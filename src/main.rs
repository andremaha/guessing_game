extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	let secret_number = rand::thread_rng().gen_range(1, 101);

	println!("The secret is: {}", secret_number);

    println!("--- GUESS THE NUMBER ---");

    loop {
    	 println!("Please type your guess here: ");

    	 let mut users_guess = String::new();

    	 io::stdin().read_line(&mut users_guess)
    	 	.expect("ERROR> Failed to read the line");

    		let users_guess: u32 = match users_guess.trim().parse() {
    			Ok(num) => num,
    			Err(_) => {
    				println!("ERROR> Only numbers please!");
    				continue;
    			},
    		};

    	 println!("Your guess was: {}", users_guess);

    	 match users_guess.cmp(&secret_number) {
    	 	Ordering::Less => println!("Too small!"),
    	 	Ordering::Greater => println!("Too big!"),
    	 	Ordering::Equal => {
    	 		println!("VICTORY!!!");
    	 		break;
    	 	}
    	 }
    }

}
