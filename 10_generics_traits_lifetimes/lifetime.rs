/*/
In Rust, lifetimes are a concept used to ensure that references to memory are valid for as long as they are used. A lifetime is essentially a scope during which a particular reference is valid. This helps prevent issues like dangling pointers or referencing memory that has already been freed.
 */

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}