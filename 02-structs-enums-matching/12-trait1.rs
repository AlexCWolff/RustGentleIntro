// trait1.rs

/* Rust structs cannot inherit from other structs; they are all unique types, and there is no sub-typing. Use traits to establish relationships between types. */

trait Show {
  // Define a trait 'Show'
    fn show(&self) -> String;
}

impl Show for i32 {
    // Implement the train 'Show' for 'i32'
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    // Implement the train 'Show' for 'f64'
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn main() {
    let answer = 42;
    let maybe_pi = 3.14;
    let s1 = answer.show();
    let s2 = maybe_pi.show();
    println!("show {}", s1);
    println!("show {}", s2);
}
// show four-byte signed 42
// show eight-byte float 3.14

/*  We added a new method to both 'i32' and 'f64' with layout and syntax roughly similar to Interfaces, but much more straightforward. Getting comfortable with Rust (and any language really) involves learning the basic traits of the standard library since these things come up time and again.

'Debug' is very common. We gave 'Person' a default implementation with the convenient '#[derive(Debug)]', but say we want a Person to display as its full name:

use std::fmt;

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}
...
    println!("{:?}", p);
    // John Smith
   
'write!' is a very useful macro; here 'f' is anything that implements 'Write', like a 'File' or 'String'. 'Display' controls how values are printed out with "{}" and is implemented just like 'Debug'. 'ToString' is automatically implemented for anything implementing 'Display', so if we implement 'Display' for 'Person' then 'p.to_string()' also works. 'Clone' defines the method 'clone', and can simply be defined with "#[derive(Clone)]" if all the fields themselves implement 'Clone'.
*/