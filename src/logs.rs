macro_rules! warning {
    ($message:expr) => {{
        use ansi_term::Colour::Yellow;
        println!("{}",
            Yellow
                .bold()
                .paint("[WARNING] ".to_owned() + $message)
                .to_string()
        )
    }};
}

macro_rules! error {
    ($message:expr) => {{
        use ansi_term::Colour::Red;
        println!("{}",
            Red
                .bold()
                .paint("[ERROR] ".to_owned() + $message)
                .to_string()
        )
    }};
}

macro_rules! epanic {
    ($message:expr) => {{
        panic!(error!($message));
    }};
}

macro_rules! info {
    ($message:expr) => {{
        use ansi_term::Colour::White;
        println!("{}",
            White
                .bold()
                .paint("[DEBUG] ".to_owned() + $message)
                .to_string()
        )
    }};
}

macro_rules! ok {
    ($message:expr) => {{
        use ansi_term::Colour::Green;
        println!("{}",
            Green
                .bold()
                .paint("OK! ".to_owned() + $message)
                .to_string()
        )
    }};
}