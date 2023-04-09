/*
Option is another enum defined by the standard library
The Option type encodes the very common scenario in which a value could be something or it could be nothing.

Rust doesn’t have the null feature

*/


fn main() {
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    match some_number {
        Some(num) => println!("The number is: {}", num),
        None => println!("No number provided."),
    }

    match no_number {
        Some(num) => println!("The number is: {}", num),
        None => println!("No number provided."),
    }
}
