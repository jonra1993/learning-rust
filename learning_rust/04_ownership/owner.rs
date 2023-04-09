/*
Each value in Rust has an owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.
 */

fn main() {

    // String that can be mutated
    let mut s = String::from("hello");  //The double colon :: operator allows us to namespace this particular from function under the String type rather than using some sort of name like string_from.

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`


    // x is still valid and wasn’t moved into y. for basic types
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);


    // A String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. 
    // data is stored on the stack
    // More reference https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
    // Rust will never automatically create “deep” copies of your data.
    let s1 = String::from("hello");  //S1 is a pointer
    //s1 was moved into s2
    let s2 = s1; //Rust considers s1 as no longer valid from here
    
    //If we do want to deeply copy the heap data of the String, not just the stack data
    let s3 = s2.clone();

    println!("s2 = {}, s3 = {}", s2, s3);

}
