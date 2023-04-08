/*
In Rust, you can declare a new variable with the same name as a previous variable. This is called variable shadowing, which means that the first variable is overshadowed by the second one. When you refer to the variable name, the compiler will use the second variable instead of the first.
The second variable will continue to overshadow the first until either the second variable is shadowed itself or the scope ends.
To shadow a variable, you can use the same variable name and repeat the `let` keyword. This allows you to declare a new variable with the same name as the old one, effectively shadowing it.
Variable shadowing can be a useful tool in Rust, allowing you to reuse variable names without worrying about conflicts with previous uses of the same name.
However, excessive use of variable shadowing can make your code harder to read and understand. Therefore, it's important to use it judiciously and sparingly, in cases where it provides clear benefits to your code.
*/

// Shadowing is different from marking a variable as mut
fn main() {
    let x: i32 = 5; // binds x to a value of 5

    // It creates a new variable x by repeating let x =, taking the original value and adding 1 so the value of x is then 6
    let x: i32 = x + 1; //first variable is shadowed "sombreado" by the second

    {
        // It creates a new variable, multiplying the previous value by 2 to give x a value of 12
        let x:i32 = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // When that scope is over, the inner shadowing ends and x returns to being 6
    println!("The value of x is: {x}");

    // Because it creates a new variable it allows change its type which is not posible with mut
    let spaces = "   ";
    println!("The value of spaces before is: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces after: {spaces}");

}