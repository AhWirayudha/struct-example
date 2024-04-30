// debug mode
#[derive(Debug)]
// struct rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Define constants
    let width = 30;
    let height = 50;

    println!("The area of the rectangle is {} square pixels.", area(width, height));

    // using tuple, its add a little bit of structure but this version is less clear cause we need to name the dimensions
    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels.", area_tuple(rect1));

    // using struct, clear and more readable
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area_struct(&rect2));

    // derived trait
    // debug mode
    println!("{:?}", rect2); // :? debug trait print struct
    // debug mode but pretty
    println!("{:#?}", rect2);
    // debug with dbg! macro, dbg! take ownership by default but we can use reference
    dbg!(&rect2); // show more detail information like line of code
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 { // borrow (&) struct instead of owning it
    rectangle.width * rectangle.height
}