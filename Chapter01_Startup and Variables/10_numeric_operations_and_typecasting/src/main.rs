fn main() {
    // addition
    let sum: i32 = 5 + 10;
    let sum: i8 = sum as i8;

    println!("sum: {}", sum);

    // subtraction
    let difference: f64 = 95.5 - 4.3;
    let difference: f32 = difference as f32;
    println!("difference: {}", difference);

    // multiplication
    let _product: i32 = 4 * 30; //adding an underscore to a variable name will suppress the warning
    

    // division
    let quotient: f64 = 56.7 / 32.2;    //we are getting the warning here
    let truncated: u32 = (-5 / 3) as u32; // Results in an overflow change it to i32 to get the truncated value

    println!("truncated: {}", truncated);

    // remainder
    let remainder: i32 = 43 % 5;

    println!("remainder: {}", remainder);
}