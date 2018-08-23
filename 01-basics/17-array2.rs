/* array2.rs

Rather than arrays, slices are often used. 
Think of these as a view into an underlying array of values. 
They otherwise behave like an array, and know their size.
Note how to write a slice's type and that it must be passed by reference.

read as: slice of i32 */
fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

fn main() {
    let arr = [10,20,30,40];
    // Don't forget the reference
    let res = sum(&arr);
    println!("sum {}", res);
}

/* Rust arrays and slices are similar to C arrays and pointers except Rust 
slices keep track of their size and will panic if you try to access outside 
that size, and you have to explicitly say that you want to pass an array as 
a slice using the '&' operator. 

'&' is 'address of' in C, in Rust it's 'borrow'. Borrowing is whenever you pass
something by reference or pass a pointer in C, it stays with the owner. */