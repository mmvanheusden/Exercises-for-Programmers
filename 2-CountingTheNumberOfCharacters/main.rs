// PLAN: First, ask for user input, ask for name or something, and then output the length of the user input.
//TODO: finish challenges
use std::io;

fn main() {
    let mut user_input = String::new(); // We store the typed text in here.
    println!("Hey! what's your name? ");

    io::stdin()// open io thingy
        .read_line(&mut user_input) // read the user input, store it in the variable.
        .expect("No string was inputted!");

    println!("You entered {}", &user_input);
    println!("Now calculating length!");
    println!("{}", return_length_of_string(&user_input))
}

// Takes a REFERENCE to a string, and returns its size.
fn return_length_of_string(s: &str) -> usize {
    s.len() - 1 // minus 1 because the enter key counts too!
}