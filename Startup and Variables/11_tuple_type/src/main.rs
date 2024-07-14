fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {y}");
    println!("The value of 3rd index of tuple is: {}", tup.2);  //Keep in mind that you cannot use tup.2 inside {}
}
