use std::io;
use coffee_machine_rs::{Coffee, Coin};

fn main() {
    let mut user_input: String = String::new();
    let stdin: io::Stdin = io::stdin();

    stdin.read_line(&mut user_input)
    .expect("Invalid input");

    let choice = Coffee::new_coffee(Coffee::from_str(user_input.trim()).unwrap());

    println!("{:?}", choice);

    println!("{:?}", Coin::value_in_cents(Coin::Penny));
}


