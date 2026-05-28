use std::io;
use rand::random_range;

fn main() {
	println!("Welcome to my guessing game!");
	println!("Enter a numerical guess.");
        
        /*

        let random_number = rand::thread_rng() 
             .gen_range(1..=100);
        
        I don't think that rand::thread_rng() works in the latest version of the rand crate, I checked docs.rs and I think I found the solution based on what I wrote below
        */

        let random_number = random_range(1..=100);

        println!("The random number is: {random_number}");

	let mut guess = String::new();
	io::stdin() 
		.read_line(&mut guess)
		.expect("Failed to read your input");
	println!("Your guess: {guess}");
}
