use std::io::{stdin, stdout, Write};

fn greet_user() -> String {
    let name = ask_name().trim().to_owned();
    format!("Hello, {}, nice to meet you!", name)
}

fn ask_name() -> String {
    print!("What is your name? ");
    stdout().flush();
    let mut buffer = String::new();
    stdin().read_line(&mut buffer);
    if buffer.trim().is_empty() { panic!("You entered nothing!") } else { buffer }
}

fn main() {
    println!("{}", greet_user());
}