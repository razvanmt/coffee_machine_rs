use coffee_machine_rs::{Coffee, CoffeeName, Coin, Resource, ResourceName};
use std::io;
use text_colorizer::*;



fn main() {
    loop {
        /*
        ### Resource Handling ###
         */

        // Initiate the resource 'water'
        let mut water = Resource::new_resource(ResourceName::Water);
        // Initiate the resource 'milk'
        let mut milk = Resource::new_resource(ResourceName::Milk);
        // Initiate the resource 'coffee'
        let mut coffee = Resource::new_resource(ResourceName::Coffee);

        /* 
        ### USER INPUT AND MATCHING ###
        */

        // Create a mutable String buffer on memory for the user input
        let mut user_input: String = String::new();
        let stdin: io::Stdin = io::stdin();
        // Read the stdin and insert it into the mut String user_input
        stdin.read_line(&mut user_input).expect("Invalid input");
        let a = user_input.trim();

        match a {
            "shut down" => break,
            "add water" => water.add_resource(500), // not working yet in the match arm
            "add milk" => milk.add_resource(500), // not working yet in the match arm
            "add coffee" => coffee.add_resource(500), // not working yet in the match arm
            "report" => println!("{:?}; {:?}; {:?};", water, milk, coffee),
            "Espresso" | "Latte" | "Cappuccino" => {
                let x = Coffee::new_coffee(Coffee::from_str(a).unwrap());
                println!("{:?}", x)
            },
            _ => eprintln!("{} This not a valid command for the Coffee Machine!", "ERROR!".red().bold())
        };

        // Using the Coffee new_coffee constructor to initialize choice variable as a variant of the struct according to the user_input
        // let choice = Coffee::new_coffee(Coffee::from_str(user_input.trim()).unwrap());

        // println!("{:?}", choice);

        // println!("{:?}", Coin::value_in_cents(Coin::Penny));
    }
}