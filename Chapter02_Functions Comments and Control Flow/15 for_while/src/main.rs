fn main() {
    countdown(23);
    count_even(100);
}

fn countdown(mut num: i32) {
    while num>0{
        num -= 1;
        println!("number is: {}", num);
    }
}

fn count_even(num:i32){
    for i in 0..num{
        if i%2 == 0{
            println!("even number is: {}", i);
        }
    }
}