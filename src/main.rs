#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect : &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle{
        width: 40,
        height: 60,
    };
    
    println!("The area of the rectangle {:#?} is {} square pixels.", rect1, rect1.area());
    // Check if rect1 can fully cover rect2 and rect3, resp.
    println!("Can rect1 fully cover rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 fully cover rect3? {}", rect1.can_hold(&rect3));
}