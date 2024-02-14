use std::io;
use std::io::Write;

//todo: revisit
/// Ask for a quote, then a name and then print the quote and the name nicely.
fn main() {
    let mut quote = String::new(); // We store the quote in here.
    let mut author = String::new(); // We store the quote in here.
    print!("Hello, what's the quote that you want to process? ");
    io::stdout().flush().unwrap(); // idk but this fixes the newline
    io::stdin()
        .read_line(&mut quote) // read the user input, store it in the variable.
        .expect("No string was inputted!");

    print!("Cool, and who made that quote? ");
    io::stdout().flush().unwrap(); // idk but this fixes the newline
    io::stdin()
        .read_line(&mut author) // read the user input, store it in the variable.
        .expect("No string was inputted!");

    println!("The quote \"{}\" was written by: {}.", quote.trim(),author.trim());
}