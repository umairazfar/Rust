fn main() {
    immutable_references();
    mutable_references();
    mutable_and_immutable();
    //dangling_references();
    normal_references();
}

fn immutable_references() {
    let x: i32 = 5;
    let y: &i32 = &x;   //here we have an immutable reference to x
    let z: &&i32 = &y;  //here we have an immutable reference to y

    println!("value of x is {x}, the value of y is {y} and the value of z is {z}");
}

fn mutable_references()
{
    let mut x: i32 = 10;
    let y: &mut i32 = &mut x;  //here we have a mutable reference to x
    *y = 20;                   //here we change the value of x
    println!("value of x is {x}");
}

fn mutable_and_immutable(){
    let mut x: i32 = 10;
    let ref1: &i32 = &x;
    let ref2: &i32 = &x;
    println!("value of ref1 is {ref1} and value of ref2 is {ref2}");    //commenting out this line will result in an error, you have to use
                                                                        //the immutable references before declaring a mutable reference
    let ref3: &mut i32 = &mut x;    
    println!("value of ref3 is {ref3}");
    *ref3 = 20;
    println!("value of x is {x}");
}

//This definition will cause error as you cannot return a reference whose owner is no longer in scope
//fn dangling_references() -> &String
//{
    //let s = String::from("hello");
    //&s
//}

fn normal_references() -> String
{
    let s = String::from("hello");
    s
}