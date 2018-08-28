// life2.rs

#[derive(Debug)]
struct A {
    // Assure Rust that the string slice always refers to static strings.
    s: &'static str
}

fn main() {
    let a = A { s: "hello dammit" };

    println!("{:?}", a);
}
// A { s: "hello dammit" }

/* This can be used to specify a string slice that is returned from a function. This works for the special case of static strings, but is very restrictive.

fn how(i: u32) -> &'static str {
    match i {
    0 => "none",
    1 => "one",
    _ => "many"
    }
}
*/