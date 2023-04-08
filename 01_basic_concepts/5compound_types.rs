/*
Compound types can group multiple values into one type
 */

fn main() {
    //TUPLE
    let tup: (i32, f64, u8) = (500, 6.4, 1); //Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("The value of one is: {one}");

    //ARRAY
    let a = [1, 2, 3, 4, 5]; //Unlike a tuple, every element of an array must have the same type. Arrays in Rust have a fixed length
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];  //The array named a will contain 5 elements that will all be set to the value 3 initially so let a = [3, 3, 3, 3, 3];
    let first = a[0];
    let second = a[1];

    println!("The value of second is: {second}");


}
