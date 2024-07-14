const PI: f32 = 3.1415;

fn main() {
    println!("Area of circle is: {}", area_of_circle(5.0));
    println!("Perimeter of circle is: {}", perimeter_of_circle(5.0));
    println!("Area of tiles is: {}", area_of_tiles(5.0, 5.0, 10.0));
}

// Calculates the area of a circle
fn area_of_circle(radius: f32) -> f32 {
    PI * radius * radius    //note that there is no semicolon here
}

/* Calculates the perimeter of a circle */
fn perimeter_of_circle(radius: f32) -> f32 {
    return 2.0 * PI * radius;   //return statement is optional but if you use it, you need to put a semicolon at the end
}

//This function calculates the area of all the tiles
fn area_of_tiles(tile_width: f32, tile_height: f32, num_tiles: f32) -> f32 {
    let area: f32 = {   //this is an expression block
        tile_width * tile_height
    };
    area * num_tiles
}
