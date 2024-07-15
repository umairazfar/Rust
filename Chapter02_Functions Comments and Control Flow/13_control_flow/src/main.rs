fn main() {
    println!("Choose an option!");
    println!("1. Area of rectangle");
    println!("2. Perimeter of rectangle");

    let mut option = String::new();
    std::io::stdin().read_line(&mut option).expect("Failed to read line");
    let option: u32 = option.trim().parse().expect("Please type a number!");

    if option == 1 {
        println!("Enter the width of the rectangle");
        let mut width = String::new();
        std::io::stdin().read_line(&mut width).expect("Failed to read line");
        let width: u32 = width.trim().parse().expect("Please type a number!");

        println!("Enter the height of the rectangle");
        let mut height = String::new();
        std::io::stdin().read_line(&mut height).expect("Failed to read line");
        let height: u32 = height.trim().parse().expect("Please type a number!");

        let area: u32 = area_of_rectangle(width, height);
        println!("The area of the rectangle is {area}");

        let statement: &str = if area <=50 { "it is a small square" } else { "it is a large square" };  //if-else as a statement
        println!("{statement}");

    } else if option == 2 {
        println!("Enter the width of the rectangle");
        let mut width = String::new();
        std::io::stdin().read_line(&mut width).expect("Failed to read line");
        let width: u32 = width.trim().parse().expect("Please type a number!");

        println!("Enter the height of the rectangle");
        let mut height = String::new();
        std::io::stdin().read_line(&mut height).expect("Failed to read line");
        let height: u32 = height.trim().parse().expect("Please type a number!");

        let perimeter: u32 = perimeter_of_rectangle(width, height);
        println!("The perimeter of the rectangle is {perimeter}");

        let statement: &str = if perimeter <=30 { "it is a small square" } else { "it is a large square" }; //if-else as a statement
        println!("{statement}");

    }
    else {
        println!("Invalid option");
    }
}

fn area_of_rectangle(width: u32, height: u32) -> u32 {
    width * height
}

fn perimeter_of_rectangle(width: u32, height: u32) -> u32 {
    2 * (width + height)
}
