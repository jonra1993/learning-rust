/*
The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function
Hash maps store their data on the heap
 */

 use std::collections::HashMap;


fn main() {
    let mut scores:HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Blue"), 25);  //Overwriting a Value
    scores.entry(String::from("Yellow")).or_insert(50); //Adding a Key and Value Only If a Key Isn’t Present
    scores.entry(String::from("Yellow")).or_insert(50); //Adding a Key and Value Only If a Key Isn’t Present

    println!("{:?}", scores);


    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
