/*
Function bodies are made up of a series of statements optionally ending in an expression.

Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value. Let’s look at some examples.
 */

fn main() {
    println!("Hello, world!");

    another_function1(); //Statement
    another_function2(75); //Statement
    print_labeled_measurement(5, 'h');  //Statement
    let new_value = increase_value(40); //
    println!("The value of new_value is: {new_value}");  //Statement

    let decreased_value = decrease_value(40); //
    println!("The value of decreased_value is: {decreased_value}");  //Statement
}

//Functions uses snake case
fn another_function1() {
    println!("Another function.");  //Statement
}


fn another_function2(x: i32) {
    println!("The value of x is: {x}");  //Statement
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");  //Statement
}

fn increase_value(value: i32) -> i32 {
    let x = 3;     //Statement
    x + value           //This expression because there are no ;
}

fn decrease_value(value: i32) -> i32 {
    let x = 3;     //Statement
    return x - value;   //This requires return because ;
}