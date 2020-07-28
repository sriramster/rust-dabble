use std::io;

// Debug annotations
#[derive(Debug)]
struct Student {
    name : String,
    age : u64,
}

// Methods example
impl Student {
    // Test method to categorize student
    fn is_high_school(self) {
        if self.age > 10 {
            println!("Yes, belongs to high school");
        }
        else {
            println!("Yes, belongs to middle school");
        }
    }
}

fn init_student(name: String, age: u64) -> Student {
    Student {
        name: name,
        age: age,
    }
}

fn main() {
    let name = String::from("hello");
    let s1 = init_student(name, 20);
    println!("{:?} ", s1);
    s1.is_high_school();
}
