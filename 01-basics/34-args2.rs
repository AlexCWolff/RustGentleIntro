// args2.rs (aka args1.rs)

// More idiomatic version
use std::env;

fn main() {
    let first = env::args().nth(1).expect("please supply an argument");
    let n: i32 = first.parse().expect("not an integer!");
    /* nth(1) gives you the second value of the iterator, and expect is like an
    unwrap with a readable message. Converting a string into a number is 
    straightforward, but you do need to specify the type of the value. This 
    program can panic, so don't make this a habit. */
}
