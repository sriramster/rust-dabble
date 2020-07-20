// External Dependencies
extern crate rand;

// Include namespaces/crates
use std::io;
use rand::Rng;
use std::cmp::Ordering;

// Main Function
fn main() {
    println!("Guess the number!");

    let s_num = rand::thread_rng().gen_range(1, 101);

    loop {
	println!("Please input your guess");
	
	let mut guess = String::new();

	io::stdin().read_line(&mut guess)
	    .expect("Failed to read line");

	//  trim to remove '/n'
	let guess: u32 = match guess.trim().parse() {
	    Ok(num) => num,
	    Err(_)  => continue,
	};
	
	println!("You guess: {}", guess);

	match guess.cmp(&s_num) {
	    Ordering::Less => println!("Small"),
	    Ordering::Greater => println!("Big"),
	    Ordering::Equal => {
		println!("Win!");
		break;
	    }
	}
    }
}
