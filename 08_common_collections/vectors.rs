/*
Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory
 */

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Vectors are implemented using generics
    let v1: Vec<i32> = Vec::new();

    // Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it.
    let v2: Vec<i32> = vec![1, 2, 3];

    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(10);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &v2 {
        println!("{i}");
    }

    let mut v3: Vec<i32> = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    println!("v3 = {:?}!", v3);
    dbg!(&v3);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    {
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        println!("row = {:?}!", row);
    }   // <- row goes out of scope and is freed here
}
