use std::io::{stdin, stdout, Write};

fn generate_quote() -> String {
    let quote = ask_question("What is the quote? ");
    let author = ask_question("Who said it? ");

    author + " says, \"" + quote.as_str() + "\""
}


fn ask_question(question: &str) -> String {
    print!("{}", question);
    stdout().flush();
    let mut buffer = String::new();
    stdin().read_line(&mut buffer);
    if buffer.trim().is_empty() { panic!("You entered nothing!") } else { buffer.trim().to_owned() }
}


fn main() {
    println!("{}", generate_quote());
}