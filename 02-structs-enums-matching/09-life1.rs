// life1.rs

// Usually structs contain values, but often they also need to contain references.
#[derive(Debug)]
struct A {
    s: &str
}

fn main() {
    let a = A { s: "hello dammit" };

    println!("{:?}", a);
}

/* 
error[E0106]: missing lifetime specifier
 --> life1.rs:5:8
  |
5 |     s: &str
  |        ^ expected lifetime parameter
  
Rust will not allow a reference to be stored without knowing its lifetime. All references are borrowed from some value, and all values have lifetimes, so the lifetime of a reference cannot be longer than the lifetime of that value. Rust cannot allow a situation where that reference could suddenly become invalid. String slices borrow from string literals like "hello" or from 'String' values. String literals exist for the duration of the whole program, i.e. 'static' lifetime.
 */