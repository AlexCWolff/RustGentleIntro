// fun4.rs

// Functions can modify their arguments with mutable references
// Similar to C, pass by reference and dereference must be explicit
// Rust is adding friction here to guide you towards returning values from functions
// &mut isn't needed much, usually only when you don't want to copy a large object
fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn main() {
    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {}", res);
}