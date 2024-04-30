fn main() {
    // Define constants
    let width = 30;
    let height = 50;

    println!("The area of the rectangle is {} square pixels.", area(width, height));

    // using tuple, its add a little bit of structure but this version is less clear cause we need to name the dimensions
    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels.", area_tuple(rect1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
