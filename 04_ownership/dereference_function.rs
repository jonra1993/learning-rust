/*
A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable.
Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

In Rust, the & operator is used to create a reference to a value.
A reference is a way to refer to the value of a variable without taking ownership of it.
 */

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); //We call the action of creating a reference borrowing

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    println!("s = '{}'", s);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// Change function will mutate the value it borrows.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
