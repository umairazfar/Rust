fn main() {
    loop_break();
    loop_returning();
}

///loop breaks when condition is met
fn loop_break() {
    let mut x: i32 = 0;
    
    'counting_for_x: loop { //label is not necessary for loops, only to keep track if there are multiple loops
        x+=1;
        if x == 5 {
            println!("x = {} hence breaking", x);
            break 'counting_for_x;  //if you put the label then use it or you will get a warning
        }
    }
}

///loop returns a value as it breaks
fn loop_returning() {
    let mut x: i32 = 0;

    let y: i32 = loop{ x=x+1;
        if x == 5 {
            println!("x = {} hence returning", x);
            break x * 2;
        }
    };
    println!("value of y is = {} which is twice of x = {}", y, x);
}
