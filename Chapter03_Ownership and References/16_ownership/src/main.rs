fn main() {
    let s1 = String::from("hello");  // s1 comes into scope
    let s2 = String::from("world");  // s2 comes into scope

    takes_ownership(s1);        // s1's value moves into the function...
                                            // ... and so is no longer valid here
    takes_ownership(s2.clone());    // s2's value remains intact and a clone is sent to the function
                                            // ... and s2 is still valid here

    //println!("{s1}"); //This will not compile
    println!("{s2}");

    let x = 5;                  // x comes into scope
 
    makes_copy(x);     // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} 