fn main() {
    countdown(23);
    count_even(100);
    countdown_in_for(10);
    showing_array_elements();
}

fn countdown(mut num: i32) {
    println!("counting down from {} to 0", num);
    while num>0{
        num -= 1;
        println!("number is: {}", num);
    }
}

fn count_even(num:i32){
    println!("Even numbers till {} are: ", num);
    for i in 0..num{
        if i%2 == 0{
            println!("even number is: {}", i);
        }
    }
}

fn countdown_in_for(num:i32){
    println!("counting down from {} to 0", num);
    for i in (0..num).rev(){
        println!("number is: {}", i);
    }
}

fn showing_array_elements(){
    println!("The elements of array are: ");
    let arr: [i32; 5] = [1,2,3,4,5];
    for element in arr{
        println!("number is: {}", element);
    }
}