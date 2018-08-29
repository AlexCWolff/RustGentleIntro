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

/* Again, this enum can only contain one of these values; its size will be the size of the largest variant. The superpower of 'match' is they know what kind of value they contain. */

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

// 'Option' and 'Result' are both enums.

// Pass the value as a reference instead
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
   
Rust is not letting you extract the string contained in the original value because of limitations on borrowed references. It did not complain about 'Number' because 'f64' implements 'Copy', 'String' does not. Recall that 'match' is picky about exact types. */

fn dump(v: &Value) {
    use Value::*;
    match *v {
        Number(n) => println!("number is {}", n),
        // Borrow a reference to the contained string
        Str(ref s) => println!("string is '{}'", s),
        Bool(b) => println!("boolean is {}", b)
    }
}
....
dump(&s);
// string is 'hello'

/* Rust is good at generating errors that have enough context for a human to fix the error without necessarily understanding the error. This is because of the exactness of matching with the borrow checker enforcing the rules. One of those rules is that you cannot pull out a value which belongs to an owning type. Experience with C++ can be a hindrance here since C++ will copy its way out of the problem, whether or not that copy makes sense. 

You will get exactly the same error if you try to pull out a string from a vector, like with '*v.get(0).unwrap()' ('*' because indexing returns references). Rust will not let you do this, although sometimes clone isn't a bad solution. Note that 'v[0]' does not work for non-copyable values like strings for precisely this reason; you must either borrow with '&v[0]' or clone with 'v[0].clone()'.

With match you can see 'Str(s) =>' as short for 'Str(s: String) =>'. A local variable, often called a binding, is created. Often the inferred type is fine when you take a value and extract its contents, but here what we need is 's: &String', and the 'ref' is a hint that ensures that we just want to borrow that string. */

// Here we want to extract the string and don't care about the enum value afterwards
impl Value {
    fn to_str(self) -> Option<String> {
        match self {
        Value::Str(s) => Some(s),
        // '_' as usual will match anything
        _ => None
        }
    }
}
...
println!("s? {:?}", s.to_str());
// s? Some("hello")
// println!("{:?}", s) // error! s has moved...

/* Naming matters: this is called 'to_str', not 'as_str'. You can write a method that only borrows the string as an 'Option<&String>' (the reference will need the same lifetime as the enum value) but you would not call it 'to_str'. */

// You can write 'to_str' like this, it is equivalent
fn to_str(self) -> Option<String> {
    if let Value::Str(s) = self {
        Some(s)
    } else {
        None
    }
}