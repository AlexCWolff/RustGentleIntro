// slice2.rs


/* Slices can be indexed, but the size of a slice is only known at run-time, 
so s[i] can cause an out-of-bounds error when running and will panic, 
and there are no exceptions in Rust. There is a slice method 'get' which does 
not panic. */
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first);
    println!("last {:?}", last);
}
// first Some(1)
// last None

/* 'last' failed, but returned something called 'None'. 'first' is fine, but is
a value wrapped in 'Some'. This is the 'Option' type, and it can be either 
'Some' or 'None'. The 'Option' type has some useful methods:
 
 println!("first {} {}", first.is_some(), first.is_none()); // first true false
 println!("last {} {}", last.is_some(), last.is_none()); // last false true
 println!("first value {}", first.unwrap()); // first value 1

If you were to unwrap 'last', you would get a panic, but you can call 'is_some' 
first to make sure - for instance, if you had a distinct no-value default:

let maybe_last = slice.get(5);
let last = if maybe_last.is_some() {
    // Note the *, the type is '&i32' so we need to dereference this to get an 'i32'
    *maybe_last.unwrap()
} else {
    -1
};

This is bulky, but 'unwrap_or' will return the value it is given if the 'Option'
was 'None'. The types must match up, so you have to make up a '&i32' with '&-1'.
Dereference to get the value as i32.

let last = *slice.get(5).unwrap_or(&-1);

Think of 'Option' as a box which may contain a value, or nothing. It may contain 
any kind of value, which is its type parameter. In this case, the full type is 
Option<&i32>, using C++-style notation for generics. It is very common for Rust 
functions to return 'Option'.*/
