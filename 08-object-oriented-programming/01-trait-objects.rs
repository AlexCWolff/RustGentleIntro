/* The chances are good that your previous programming language implemented Object-Oriented Programming (OOP) in a particular way:

* 'classes' act as factories for generating objects (often called instances) and define unique types.
* Classes may inherit from other classes (their parents), inheriting both data (fields) and behaviour (methods)
* If B inherits from A, then an instance of B can be passed to something expecting A (subtyping)
* An object should hide its data (encapsulation), which can only be operated on with methods.

Object-oriented design is then about identifying the classes (the 'nouns') and the methods (the 'verbs') and establishing relationships between them (is-a and has-a). Rust-flavoured object-orientation is pretty different from what you may be used to: Rust data aggregates (structs, enums and tuples) are dumb, you can define methods on them, and you can make the data private, but they are all unrelated types. There is no subtyping and no inheritance of data (apart from the specialized case of 'Deref' coercions).

The relationships between various data types in Rust are established using traits. A large part of learning Rust is understanding how the standard library traits operate, because that's the web of meaning that glues all the data types together. Traits are interesting because there's no one-to-one correspondence between them and concepts from mainstream languages, it depends if you're thinking dynamically or statically. In the dynamic case, they're rather like Java or Go interfaces. Consider the example first used to introduce traits: */

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

// A little program with big implications
fn main() {
    let answer = 42;
    let maybe_pi = 3.14;
    let v: Vec<&Show> = vec![&answer,&maybe_pi];
    for d in v.iter() {
        println!("show {}",d.show());
    }
}
// show four-byte signed 42
// show eight-byte float 3.14

/* This is a case where Rust needs some type guidance; specifically we want a vector of references to anything that implements 'Show'. Now note that 'i32' and 'f64' have no relationship to each other, but they both understand the 'show' method because they both implement the same trait. This method is virtual, because the actual method has different code for different types, and yet the correct method is invoked based on runtime information. These references are called trait objects. That is how you can put objects of different types in the same vector. If you come from a Java, C#, or Go background, you can think of 'Show' as acting like an interface. */

// A refinement of this example, we box the values. 
// A box contains a reference to data allocated on the heap, and acts like a reference; it's a smart pointer. 
// When boxes go out of scope, 'Drop' kicks in, then that memory is released.
let answer = Box::new(42);
let maybe_pi = Box::new(3.14);

let show_list: Vec<Box<Show>> = vec![question,answer];
for d in &show_list {
    println!("show {}",d.show());
}
// The difference is that you can now take this vector, pass it as a reference or give it away without having to track any borrowed references. 