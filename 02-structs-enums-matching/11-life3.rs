// life3.rs

// We can specify that the lifetime of the reference is at least as long as that of the struct itself.
#[derive(Debug)]
struct A <'a> {
    s: &'a str
}

fn main() {
    let s = "I'm a little string".to_string();
    let a = A { s: &s };

    println!("{:?}", a);
}

/* Lifetimes are conventionally called 'a','b',etc but you could just as well called it 'me'. Our 'a' struct and the 's' string are bound by a strict contract, 'a' borrows from 's' and cannot outlive it. With this struct definition, we would like to write a function that returns an 'A' value:

fn makes_a() -> A {
    let string = "I'm a little string".to_string();
    A { s: &string }
}

= help: this function's return type contains a borrowed value,
 but there is no value for it to be borrowed from
= help: consider giving it a 'static lifetime

// 'rustc' is giving advice, so we follow it
fn makes_a() -> A<'static> {
    let string = "I'm a little string".to_string();
    A { s: &string }
}

8 |      A { s: &string }
  |              ^^^^^^ does not live long enough
9 | }
  | - borrowed value only lives until here

There is no way that this could safely work, because 'string' will be dropped when the function ends, and no reference to 'string' can outlast it. Think of lifetime parameters as being part of the type of a value. Sometimes it seems like a good idea for a struct to contain a value and a reference that borrows from that value. It's almost impossible because structs must be moveable, and any move will invalidate the reference. It isn't necessary to do this; for instance, if your struct has a string field, and needs to provide slices, then it could keep indices and have a method to generate the actual slices.
*/