 use std::{fs::File, io::ErrorKind};


fn main(){
    let result = divide(10, 2);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(x / y)
    }
}