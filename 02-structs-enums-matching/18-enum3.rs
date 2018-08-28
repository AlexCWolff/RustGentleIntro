// enum3.rs

// Rust enums in their full form are like more powerful C unions.

// Store different values in a type-safe way
#[derive(Debug)]
enum Value {
    Number(f64),
    Str(String),
    Bool(bool)
}

fn main() {
    use Value::*;
    let n = Number(2.3);
    let s = Str("hello".to_string());
    let b = Bool(true);

    println!("n {:?} s {:?} b {:?}", n,s,b);
}
// n Number(2.3) s Str("hello") b Bool(true)

/* Again, this enum can only contain one of these values; its size will be the size of the largest variant.

So far, not really a supercar, although it's cool that enums know how to print themselves out. But they also know how what kind of value they contain, and that is the superpower of match: */

fn eat_and_dump(v: Value) {
    use Value::*;
    match v {
        Number(n) => println!("number is {}", n),
        Str(s) => println!("string is '{}'", s),
        Bool(b) => println!("boolean is {}", b)
    }
}
....
eat_and_dump(n);
eat_and_dump(s);
eat_and_dump(b);
//number is 2.3
//string is 'hello'
//boolean is true

/* (And that's what Option and Result are - enums.)

We like this eat_and_dump function, but we want to pass the value as a reference, because currently a move takes place and the value is 'eaten': */

fn dump(v: &Value) {
    use Value::*;
    match *v {  // type of *v is Value
        Number(n) => println!("number is {}", n),
        Str(s) => println!("string is '{}'", s),
        Bool(b) => println!("boolean is {}", b)
    }
}
/*
error[E0507]: cannot move out of borrowed content
  --> enum3.rs:12:11
   |
12 |     match *v {
   |           ^^ cannot move out of borrowed content
13 |     Number(n) => println!("number is {}",n),
14 |     Str(s) => println!("string is '{}'",s),
   |         - hint: to prevent move, use `ref s` or `ref mut s`
   
There are things you cannot do with borrowed references. Rust is not letting you extract the string contained in the original value. It did not complain about Number because it's happy to copy f64, but String does not implement Copy.

I mentioned earlier that match is picky about exact types; here we follow the hint and things will work; now we are just borrowing a reference to that contained string. */

fn dump(v: &Value) {
    use Value::*;
    match *v {
        Number(n) => println!("number is {}", n),
        Str(ref s) => println!("string is '{}'", s),
        Bool(b) => println!("boolean is {}", b)
    }
}
....
dump(&s);
// string is 'hello'

/* Before we move on, filled with the euphoria of a successful Rust compilation, let's pause a little. rustc is unusually good at generating errors that have enough context for a human to fix the error without necessarily understanding the error.

The issue is a combination of the exactness of matching, with the determination of the borrow checker to foil any attempt to break the Rules. One of those Rules is that you cannot yank out a value which belongs to some owning type. Some knowledge of C++ is a hindrance here, since C++ will copy its way out of the problem, whether that copy even makes sense. You will get exactly the same error if you try to pull out a string from a vector, say with *v.get(0).unwrap() (* because indexing returns references.) It will simply not let you do this. (Sometimes clone isn't such a bad solution to this.)

(By the way, v[0] does not work for non-copyable values like strings for precisely this reason. You must either borrow with &v[0] or clone with v[0].clone())

As for match, you can see Str(s) => as short for Str(s: String) =>. A local variable (often called a binding) is created. Often that inferred type is cool, when you eat up a value and extract its contents. But here we really needed is s: &String, and the ref is a hint that ensures this: we just want to borrow that string.

Here we do want to extract that string, and don't care about the enum value afterwards. _ as usual will match anything. */

impl Value {
    fn to_str(self) -> Option<String> {
        match self {
        Value::Str(s) => Some(s),
        _ => None
        }
    }
}
...
println!("s? {:?}", s.to_str());
// s? Some("hello")
// println!("{:?}", s) // error! s has moved...

/* Naming matters - this is called to_str, not as_str. You can write a method that just borrows that string as an Option<&String> (The reference will need the same lifetime as the enum value.) But you would not call it to_str.

You can write to_str like this - it is completely equivalent: */

fn to_str(self) -> Option<String> {
    if let Value::Str(s) = self {
        Some(s)
    } else {
        None
    }
}