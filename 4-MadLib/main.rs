use std::io::{stdin, stdout, Write};

use rand::prelude;
use rand::prelude::IndexedRandom;

const FIRST: &[&str] = &["Do you", "I can"];
const SECOND: &[&str] = &["my", "your"];
const THIRD: &[&str] = &["? That's hilarious!", "... What!?", ". That's weird....", ". Interesting..."];


fn generate_story() -> String {
    let noun = ask_question("Enter a noun (zelfst. naamwoord): ");
    let verb = ask_question("Enter a verb (werkwoord): ");
    let adjective = ask_question("Enter an adjective (bijv. naamwoord): ");
    let adverb = ask_question("Enter an adverb (bijwoord): ");

    format!("{} {} {} {} {} {}{}", choose_random(&FIRST), verb, choose_random(&SECOND), adjective, noun, adverb, choose_random(&THIRD))
}

fn ask_question(question: &str) -> String {
    print!("{}", question);
    stdout().flush();
    let mut buffer = String::new();
    stdin().read_line(&mut buffer);
    if buffer.trim().is_empty() { panic!("You entered nothing!") } else { buffer.trim().to_owned() }
}

fn choose_random(list: &[&str]) -> String {
    list.choose(&mut prelude::thread_rng()).unwrap().to_string()
}


fn main() {
    println!("{}", generate_story());
}