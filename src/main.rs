use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Enter the first number:");
    let num1 = read_number();

    println!("Enter the second number:");
    let num2 = read_number();

    match num1.cmp(&num2) {
        Ordering::Greater => println!("The first number ({}) is bigger.", num1),
        Ordering::Less => println!("The second number ({}) is bigger.", num2),
        Ordering::Equal => println!("Both numbers are equal."),
    }
}

fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}
