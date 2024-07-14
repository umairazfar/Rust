use std::io;
fn main() {
    const PI: f32 = 3.1415;
    let mut input: String = String::new();

    println!("Enter the radius of the circle: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: f32 = input.trim().parse().expect("Please type a number!");

    let area: f32 = PI * input * input;
    println!("The area of the circle is: {}", area);
}
