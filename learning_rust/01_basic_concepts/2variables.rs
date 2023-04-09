fn main() {    
    let mut x: i32 = 5; //without mut it is inmutable  y default
    let y: i32 = 3; //This is inmutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;   //This is a constant variable

    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
}