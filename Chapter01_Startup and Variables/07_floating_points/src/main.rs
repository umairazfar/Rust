fn main() {
    let acc: f32 = 5.5;
    let mass: f32 = 10.32;
    let force: f32 = acc * mass;
    println!("The mass = {} kg, the acceleration = {} m/s^2, the force = {} Newtons", mass, acc, force);

    let val1: f64 = 23.000059875;
    let val2: f64 = 24.000059875;
    let val3: f64 = 25.000059875;
    println!("value1: {}, value2: {}, value3: {}", val1, val2, val3);

    let val4: f64 = val1 + val2 + val3;
    println!("sum is: {}", val4);
}
