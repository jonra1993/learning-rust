fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    // loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop.
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;  //will exit the inner loop only
            }
            if count == 2 {
                break 'counting_up; //will exit the outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}