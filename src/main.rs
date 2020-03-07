extern crate rand;

use std::io;
use rand::Rng;

fn main() {
	let secret_number = rand::thread_rng().gen_range(1, 101);

	println!("The secret is: {}", secret_number);

	return;


    println!("--- GUESS THE NUMBER ---");

    println!("Please type your guess here: ");

    let mut users_guess = String::new();

    io::stdin().read_line(&mut users_guess)
    	.expect("Failed to read the line");

    println!("Your guess was: {}", users_guess);
}
