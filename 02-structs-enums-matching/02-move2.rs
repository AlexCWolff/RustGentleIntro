// move2.rs

// Re-writing with a function call reveals the same error
fn dump(s: String) {
    println!("{}", s);
}

fn main() {
    let s1 = "hello dolly".to_string();
    dump(s1);
    println!("s1 {}", s1); // <---error: 'value used here after move'
}

/* You may pass a reference to that string, or explicitly copy it using its 
clone method. Generally, the first is the better way to go. */

fn dump(s: &String) {
    println!("{}", s);
}

fn main() {
    let s1 = "hello dolly".to_string();
    dump(&s1);
    println!("s1 {}", s1);
}

/* You'll rarely see a plain 'String' reference like this, since to pass a
string literal is really ugly and involves creating a temporary string. 

dump(&"hello world".to_string());
*/

// The best way to declare that function
fn dump(s: &str) {
    println!("{}", s);
}

/* Both 'dump(&s1)' and 'dump("hello world")' work properly now. (Here 'Deref' 
coercion kicks in and Rust will convert '&String' to '&str' for you). Assignment
of a non-Copy value moves the value from one location to another, otherwise Rust 
would be forced to implicitly do a copy and break its promise to make 
allocations explicit. */