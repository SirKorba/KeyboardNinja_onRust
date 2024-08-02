slint::include_modules!();

use keyboard_ninja::run;


fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to start programm:\nError: {:?}", e);
        }
    };
}
