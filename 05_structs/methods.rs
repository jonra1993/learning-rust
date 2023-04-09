/*
Methods are similar to functions: we declare them with the fn keyword and a name
Unlike functions, methods are defined within the context of a struct (or an enum or a trait object
 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//  Everything within this impl block will be associated with the Rectangle type
impl Rectangle {
    // All of these are called associated functions
    fn get_area(&self) -> u32 {
        self.width * self.height
    }
    fn has_width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.get_area()
    );

    if rect1.has_width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
