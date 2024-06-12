use std::io::{stdin, stdout, Write};

fn get_char_number() -> String {
    // https://stackoverflow.com/questions/54056268/temporary-value-is-freed-at-the-end-of-this-statement#comment131362495_54056716
    let input = &ask_input().trim().to_owned();
    let length = &input.len();
    format!("{input} has {length} characters.")
}


fn ask_input() -> String {
    print!("What is the input string? ");
    stdout().flush();
    let mut buffer = String::new();
    stdin().read_line(&mut buffer);
    if buffer.trim().is_empty() { panic!("You entered nothing!") } else { buffer }
}


fn main() {
    println!("{}", get_char_number());
}