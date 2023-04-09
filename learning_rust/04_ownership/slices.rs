/*
A string slice is a reference to part of a String
 */

fn main() {
    let s = String::from("hello world");

    // Rather than a reference to the entire String, hello is a reference to a portion of the String
    let hello = &s[0..5];
    let world = &s[6..11];
    let slice1 = &s[0..2];
    let slice2 = &s[..2]; //It is the same as slice1

    let len = s.len();
    let slice3 = &s[3..len];
    let slice4 = &s[3..]; ////It is the same as slice3

    let first_word = first_word(&s);
    println!("first_word = {}", first_word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
