/*
The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type. 
 */



fn main() {
    let mut s1 = String::new();
    s1.push_str("bar");
    s1.push('1');
    println!("s1 is {s1}");

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a str literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // Concatenate
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");   //This uses format macro to concatenate
    println!("s is {s}");

    let hello = "Здравствуйте";
    let s = &hello[0..4];

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
