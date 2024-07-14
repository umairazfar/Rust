fn main() {
    let x: i32 = 5;
    let x: i32 = x+1;   //This creates a new variable at a separate memory location with the value of x+1
    println!("The value of x in main scope is: {}", x); 
    {
        let x: i32 = x+1;   //this is a new variable with the value of x+1 within a new scope. Now x in the main scope will remain hidden
        println!("The value of x in the inner scope is: {}", x);
        let x: i32 = x+1;   //we are doing shadowing again but of x in the inner scope
        println!("The value of x in the inner scope is: {}", x);
        {
            let x: i32 = x+1;   //Again, shadowing of x in the inner scope and it will not use the value of x in the main scope
            println!("The value of x in the innermost scope is: {}", x);
        }
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x in main scope is: {}", x);
}
