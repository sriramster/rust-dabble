// Function passing styles in Rust
use std::io;

// Dangling reference for Strin s. Here 's' goes out of scope
// fn return_str() -> &String { 
//     let s = String::new();
//     s = "Hello World from return_str"; // 's' alive
//     &s 
// } // 's' goes out of scope. Problem of ownership in rust 

fn mut_ref(s: &mut String) {
    s.push_str("I pushed mut_ref");
    println!("{}", s);
}

fn return_a_uint32() -> u32 {
    let j : u32 = 100;
    return j
}

fn print_string_by_reference(s: &String) {
    println!("{}", s);
}

fn print_int_by_reference(s: &u32) {
    println!("{}", s);
}

fn main() {
    println!("Testing for print by reference");

    let mut s = String::new();

    io::stdin().read_line(&mut s)
        .expect("Failed to read input");

    print_string_by_reference(&s);

    // Shawdowing
    let s = 10;
    print_int_by_reference(&s);

    let s = return_a_uint32();
    println!("{}", s);

    let mut s = String::from("Testing mut ref ");
    mut_ref(&mut s);
}
