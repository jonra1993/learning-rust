/*
Each value in Rust has an owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.
 */

fn main() {
    // s is not valid here, it’s not yet declared
    {
        let s = "hello"; // s is valid from this point forward   (string literals)

        // do stuff with s
    } // this scope is now over, and s is no longer valid

}
