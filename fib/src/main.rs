// Fibonacci Series Rust
// no recursion, simple fib which crashes on Integer overflow

use std::io;

fn main() {
    let mut n1 : u64 = 0;
    let mut n2 : u64 = 1;
    let mut tmp : u64  = 0;
    
    println!("Enter the fib series depth");

    let mut depth = String::new();
    let mut cnt :u32 = 0;

    io::stdin().read_line(&mut depth)
        .expect("Failed to read input");

    //  Shadowing
    let depth :u32 = depth.trim().parse()
        .expect("Please enter nujmber");
    
    if depth == 0 {
        println!("Sum : {}", n1);
    }
    else if depth == 1 {
        println!("Sum : {}", n2);
    }
    else {
        loop  {
            tmp = n1 + n2;
            n1 = n2;
            n2 = tmp;
            cnt = cnt + 1;
            if cnt == depth {
                break;
            }
        }
    }
    println!("Sum : {}", tmp);
    
}
