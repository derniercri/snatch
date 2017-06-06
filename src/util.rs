use std::io;
use std::io::Write;

pub fn prompt_user(prompt: &str) -> String {
    warning!(prompt);
    io::stdout().flush().expect("Couldn't flush stdout!");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .ok()
        .expect("Couldn't read line!");
    String::from(user_input.trim())
}