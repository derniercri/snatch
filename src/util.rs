use ansi_term::Style;

use std::io;
use std::io::Write;

pub fn prompt_user(style: Style, prompt: &str) -> String {
    print!("{} ", style.paint(prompt));
    io::stdout()
        .flush()
        .expect("[ERROR] Couldn't flush stdout!");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .ok()
        .expect("[ERROR] Couldn't read line!");
    String::from(user_input.trim())
}