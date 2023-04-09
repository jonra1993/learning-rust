/*
A struct, or structure, is a custom data type that lets you package together and
name multiple related values that make up a meaningful group.

A struct is like an object’s data attributes
The struct definition is like a general template for the type
 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Rust also supports structs that look similar to tuples, called tuple structs.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com"); //change email

    let user2 = build_user(
        String::from("anotheremail2@example.com"),
        String::from("username123"),
    );

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


    println!("user2 is {:?}", user2);
    dbg!(&rect1);

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
