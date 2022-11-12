#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect = Rectangle{
        width: 50,
        height: 30,
    };
    
    println!("The area of the rectangle {:#?} is {} square pixels.", rect,area(&rect));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
