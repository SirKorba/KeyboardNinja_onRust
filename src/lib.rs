slint::include_modules!();

use rand::{Rng, thread_rng};
use std::fs;
use std::io::Write;
use std::str;


pub fn run() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let cui = ui.clone_strong();
    let sui = ui.clone_strong();

    start_app();
    char_mode(cui);
    sentence_mode(sui);

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

    example_file.write_all("It is example sentence.
I love programming and learning.
Rust is very interesting language!"
    .as_bytes()).unwrap();

    example_file2.write_all("It's example sentence from second file!"
    .as_bytes()).unwrap();
}

fn char_mode(ui: AppWindow) {
    
    let mut count_correct:i32 = 0;
    let mut count_incorrect:i32 = 0;

    ui.set_char(rand_char().into());

    ui.on_key_accept({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            if ui.get_event() == ui.get_char() {
                count_correct += 1;

                ui.set_char(rand_char().into());
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
                count_correct = 0;
                count_incorrect = 0;
                ui.set_count_correct(count_correct);
                ui.set_count_incorrect(count_incorrect);
                ui.set_char(rand_char().into());
            }
        }
    });

}

fn sentence_mode(ui: AppWindow) {

    let mut count_correct = 0;
    let mut count_incorrect = 0;

    keyboard_visual(ui.clone_strong());

    ui.set_correct_sentence(rand_sentence().into());

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
                ui.set_correct_sentence(rand_sentence().into());
            } else {
                ui.set_is_correct_sentence(false);
                ui.set_keyboard_event("Enter".into());
                count_incorrect += 1;
            }

            ui.set_count_correct(count_correct);
            ui.set_count_incorrect(count_incorrect);

            if !ui.get_is_sentence_mode() {
                count_correct = 0;
                count_incorrect = 0;
                ui.set_count_correct(count_correct);
                ui.set_count_incorrect(count_incorrect);
                ui.set_correct_sentence(rand_sentence().into());
            }
        }
    })
}

fn keyboard_visual(ui: AppWindow) {

    let mut last_string:String = String::new();

    ui.on_keyboard_key_accept({
        let ui_handle = ui.as_weak();
        move || {

            let ui = ui_handle.unwrap();
            let edit_string = ui.get_edit_string().to_string();

            if edit_string >= last_string {  
                let mut ch:&str = &edit_string[edit_string.len()-1..].to_uppercase();

                if ch == " " {
                    ch = "Space";
                }

                ui.set_keyboard_event(ch.into());
            } else {
                ui.set_keyboard_event("BSpace".into());
            }

            last_string = edit_string;
        }
    })
}

fn rand_char() -> String {
    let en_chars = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
                                "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];

    let ch = en_chars[thread_rng().gen_range(0..26)];

    let ch = match thread_rng().gen_range(0..2) {                        // case
        1 => ch.to_uppercase(),
        0 => ch.to_string(),
        _ => ch.to_string()
    };

    ch
}

fn rand_sentence() -> String {

    let en_path = "texts/en/";
    let mut texts = Vec::new();

    match fs::read_dir(en_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            texts.extend(path)
        },
    }

    let file = &texts[thread_rng().gen_range(0..texts.len())];
    let path:String = format!("{}{}", en_path, file.file_name().into_string().unwrap());

    let sentences = fs::read_to_string(path).unwrap();
    let sentences:Vec<&str> = sentences.split("\n").collect();

    let sentence = sentences[thread_rng().gen_range(0..sentences.len())];

    String::from(sentence)
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
// }