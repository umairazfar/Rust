use std::io;

fn main() {
    println!("We will be taking two inputs from the user and then add them together");

    let mut var1: String = String::new();
    let mut var2: String = String::new();

    println!("Enter first number: ");

    io::stdin()
        .read_line(&mut var1)
        .expect("Failed to read line");

    let var1: i32 = var1.trim().parse().expect("Please type a number!");

    println!("Enter second number: ");

    io::stdin()
        .read_line(&mut var2)
        .expect("Failed to read line");

    let var2: i32 = var2.trim().parse().expect("Please type a number!");

    let sum: i32 = var1 + var2;

    println!("The sum is: {}", sum);
}
