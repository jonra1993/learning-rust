/*
A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. 
Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

In Rust, the & operator is used to create a reference to a value. 
A reference is a way to refer to the value of a variable without taking ownership of it.
 */

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}