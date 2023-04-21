use std::io;
use coffee_machine_rs::{Coffee, Coin};

fn main() {
    // Create a mutable String buffer on memory for the user input
    let mut user_input: String = String::new();
    let stdin: io::Stdin = io::stdin();

    // Read the stdin and insert it into the mut String user_input
    stdin.read_line(&mut user_input)
    .expect("Invalid input");

    // Using the Coffee new_coffee constructor to initialize choice variable as a variant of the struct according to the user_input
    let choice = Coffee::new_coffee(Coffee::from_str(user_input.trim()).unwrap());

    println!("{:?}", choice);

    println!("{:?}", Coin::value_in_cents(Coin::Penny));
}


