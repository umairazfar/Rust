fn main() {
    let mut num: i32 = 1000;
    println!("value of num is {}", num);

    num = 23_000;   //using alternate way to assign value when writing a large number
    println!("value of num is {}", num);

    num = 0xff;     //hexadecimal value, 15x16^1 + 15x16^0 = 255
    println!("value of num is {}", num);

    num = 0o77;     //octal value, 7x8^1 + 7x8^0 = 63
    println!("value of num is {}", num);

    num = 0b0000_1111;   //binary value, 1x2^3 + 1x2^2 + 1x2^1 + 1x2^0 = 15
    println!("value of num is {}", num);

    let byte_num: u8 = b'A';
    println!("value of byte_num is {}, which is the ASCII value of 'A'", byte_num);
}
