// struct3.rs

// A 'Person' method that takes a reference self argument
impl Person {
    ...
    // 'self' is used explicitly and passed as a reference
    // Think of '&self' as short for 'self: &Person'
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

}
...
    println!("fullname {}", p.full_name());
// fullname John Smith

// The keyword 'Self' refers to the struct type, you can mentally substitute 'Person' for 'Self'
fn copy(&self) -> Self {
    Self::new(&self.first_name,&self.last_name)
}

// Methods may allow the data to be modified using a mutable self argument
fn set_first_name(&mut self, name: &str) {
    self.first_name = name.to_string();
}

// The data will move into the method when a plain self argument is used
fn to_tuple(self) -> (String,String) {
    (self.first_name, self.last_name)
}

 /* Try that with '&self', structs will not let go of their data easily. Note that after 'v.to_tuple()' is called, 'v' has moved and is no longer available.  To summarize:
no self argument: you can associate functions with structs, like the 'new' "constructor".
&self argument: can use the values of the struct, but not change them
&mut self argument: can modify the values
self argument: will consume the value, which will move.
If you try to do a debug dump of a Person, you will get an informative error:

error[E0277]: the trait bound `Person: std::fmt::Debug` is not satisfied
  --> struct2.rs:23:21
   |
23 |     println!("{:?}", p);
   |                     ^ the trait `std::fmt::Debug` is not implemented for `Person`
   |
   = note: `Person` cannot be formatted using `:?`; if it is defined in your crate,
    add `#[derive(Debug)]` or manually implement it
   = note: required by `std::fmt::Debug::fmt`
The compiler is giving advice, so we put '#[derive(Debug)]' in front of 'Person', and now there is sensible output.

Person { first_name: "John", last_name: "Smith" }

The directive makes the compiler generate a Debug implementation. It's good practice to do this for structs so they can be printed out or written as a string using 'format!'. */