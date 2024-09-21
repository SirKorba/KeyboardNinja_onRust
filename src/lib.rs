slint::include_modules!();

use rand::{Rng, thread_rng};
use std::fs;
use std::io::Write;
use std::str;


pub fn run() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    start_app();
    char_mode(ui.clone_strong());
    sentence_mode(ui.clone_strong());

    ui.run()
}

fn start_app() {

    match fs::create_dir("texts") {
        Ok(_) => (),
        Err(er) => eprintln!("Error: {:?}", er),
    }

    match fs::create_dir("texts/ru") {
        Ok(_) => (),
        Err(er) => eprintln!("Error: {:?}", er),
    }

    match fs::create_dir("texts/en") {
        Ok(_) => (),
        Err(er) => eprintln!("Error: {:?}", er),
    }

    let mut example_file = fs::File::create("texts/en/example.txt").unwrap();
    let mut example_file2 = fs::File::create("texts/en/example2.txt").unwrap();
    let mut example_file3 = fs::File::create("texts/ru/example.txt").unwrap();

    example_file.write_all("It is example sentence.
I love programming and learning.
Rust is very interesting language!"
    .as_bytes()).unwrap();

    example_file2.write_all("It's example sentence from second file!"
    .as_bytes()).unwrap();

    example_file3.write_all("Привет, друг!
Просто пример сообщения для режима предложений на русском языке."
    .as_bytes()).unwrap();
}


fn char_mode(ui: AppWindow) {
    
    let mut count_correct:i32 = 0;
    let mut count_incorrect:i32 = 0;

    ui.set_char(rand_char(&ui).into());

    ui.on_change_lang({
        let ui_handle = ui.as_weak();

        move || {
            let ui = ui_handle.unwrap();
            ui.set_char(rand_char(&ui).into());
        }
    });

    ui.on_key_accept({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            if ui.get_event() == ui.get_char() {
                count_correct += 1;

                ui.set_char(rand_char(&ui).into());
                ui.set_is_correct_event(true);

            } else if ui.get_event() == "Shift" {
                ()
            } else {
                count_incorrect += 1;
                ui.set_is_correct_event(false);
            }
            
            ui.set_count_correct(count_correct);
            ui.set_count_incorrect(count_incorrect);

            if !ui.get_is_char_mode() {
                reset_char_mode(&ui, &mut count_correct, &mut count_incorrect);
            }
        }
    });

}

fn reset_char_mode(ui: &AppWindow, count_correct: &mut i32, count_incorrect: &mut i32) {
    *count_correct = 0;
    *count_incorrect = 0;
    ui.set_count_correct(*count_correct);
    ui.set_count_incorrect(*count_incorrect);
    ui.set_char(rand_char(&ui).into());
}

fn sentence_mode(ui: AppWindow) {

    let mut count_correct = 0;
    let mut count_incorrect = 0;

    keyboard_visual(ui.clone_strong());

    ui.set_correct_sentence(rand_sentence(&ui).into());

    ui.on_change_lang({
        let ui_handle = ui.as_weak();

        move || {
            let ui = ui_handle.unwrap();
            ui.set_correct_sentence(rand_sentence(&ui).into());
        }
    });

    ui.on_sentence_accept({
        let ui_handle = ui.as_weak();

        move || {
            let ui = ui_handle.unwrap();
            let mut _input_sentence = String::new();

            _input_sentence = ui.get_input_sentence().into();

            if _input_sentence == ui.get_correct_sentence().to_string() {
                ui.set_keyboard_event("Enter".into());
                count_correct += 1;
                ui.set_is_correct_sentence(true);
                ui.set_correct_sentence(rand_sentence(&ui).into());
            } else {
                ui.set_is_correct_sentence(false);
                ui.set_keyboard_event("Enter".into());
                count_incorrect += 1;
            }

            ui.set_count_correct(count_correct);
            ui.set_count_incorrect(count_incorrect);

            if !ui.get_is_sentence_mode() {
                reset_sentence_mode(&ui, &mut count_correct, &mut count_incorrect);
            }
        }
    })
}

fn reset_sentence_mode(ui: &AppWindow, count_correct: &mut i32, count_incorrect: &mut i32) {
    *count_correct = 0;
    *count_incorrect = 0;
    ui.set_count_correct(*count_correct);
    ui.set_count_incorrect(*count_incorrect);
    ui.set_correct_sentence(rand_sentence(&ui).into());
}

fn keyboard_visual(ui: AppWindow) {

    let mut last_string:String = String::new();

    ui.on_keyboard_key_accept({
        let ui_handle = ui.as_weak();
        move || {

            let ui = ui_handle.unwrap();
            let edit_string = ui.get_edit_string().to_string();

            if edit_string >= last_string {  
                let mut ch = edit_string.chars().last().unwrap().to_string().to_uppercase();

                if ch == " " {
                    ch = "Space".to_string();
                }

                ui.set_keyboard_event(ch.into());
            } else {
                ui.set_keyboard_event("BSpace".into());
            }

            last_string = edit_string;
        }
    })
}

fn rand_char(ui: &AppWindow) -> String {
    let en_chars = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
                                "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    let ru_chars = ["а", "б", "в", "г", "д", "е", "ё", "ж", "з", "и", "й", "к", "л", "м", "н", "о", "п",
                                "р", "с", "т", "у", "ф", "х", "ц", "ч", "ш", "щ", "ъ", "ы", "ь", "э", "ю", "я"];

    let ch:&str;
    let lang:String = ui.get_lang().parse().unwrap();

    if lang == "en" {
        ch = en_chars[thread_rng().gen_range(0..26)];
    } else {
        ch = ru_chars[thread_rng().gen_range(0..31)];
    }

    let ch = match thread_rng().gen_range(0..2) {                        // case
        1 => ch.to_uppercase(),
        0 => ch.to_string(),
        _ => ch.to_string()
    };

    ch
}

fn rand_sentence(ui: &AppWindow) -> String {

    let lang:String = ui.get_lang().parse().unwrap();
    let path = format!("texts/{}/", lang);
    let mut texts = Vec::new();

    match fs::read_dir(path.clone()) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for p in paths {
            texts.extend(p)
        },
    }

    let file = &texts[thread_rng().gen_range(0..texts.len())];
    let p:String = format!("{}{}", path, file.file_name().into_string().unwrap());

    let sentences = fs::read_to_string(p).unwrap();
    let sentences:Vec<&str> = sentences.split("\n").collect();

    let sentence = sentences[thread_rng().gen_range(0..sentences.len())];

    String::from(sentence)
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
// }