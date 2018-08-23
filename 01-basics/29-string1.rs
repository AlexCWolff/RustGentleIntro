// string1.rs

/* The 'String' type, like 'Vec', allocates dynamically and is resizeable, 
like C++'s 'std::string'. String literals however are a '&str', string slice */
fn dump(s: &str) {
    println!("str '{}'", s);
}

fn main() {
    let text = "hello dolly";  // the string slice
    let s = text.to_string();  // it's now an allocated string

    dump(text);
    dump(&s);
}

 // The borrow operator can coerce 'String' into '&str'.