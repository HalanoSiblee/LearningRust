fn main() {
    let randomstring = String::from("Hello, World!");
    let reversed_string: String = randomstring.chars().rev().collect(); //Okay I guess compiler will use std iter no idea ?
    println!("{}", reversed_string); //< now we printing our reversed+string
}
/*
//With his one we using system argument
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a string as a command line argument.");
        return;
    }

    let input_string = &args[1];
    let reversed_string: String = input_string.chars().rev().collect();

    println!("Og String: {}", input_string);
    println!("Rev String: {}", reversed_string);
}
*/
