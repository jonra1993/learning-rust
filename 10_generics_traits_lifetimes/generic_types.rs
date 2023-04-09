/*
We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.
 */

 struct Point<T, U> {
    x: T,
    y: U,
}


impl<T, U> Point<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }
}


fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("integer_and_float.x = {}", integer_and_float.get_x());


    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}