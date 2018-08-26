// ref1.rs

/* You should always prefer to keep references to the original data and 'borrow'
it, but a reference must not outlive the owner. Rust is a block-scoped language, 
so variables only exist for the duration of their block.

{
    let a = 10;
    let b = "hello";
    {
        let c = "hello".to_string();
        // a,b and c are visible
    }
    // the string c is dropped
    // a,b are visible
    for i in 0..a {
        let b = &b[1..];
        // original b is no longer visible - it is shadowed.
    }
    // the slice b is dropped
    // i is _not_ visible!
}

Loop variables like 'i' are a little different, they are only visible in the 
loop block. It is not an error to create a new variable using the same name 
('shadowing'), but it can be confusing.

When a variable 'goes out of scope' it is dropped. Any memory used is reclaimed
and any other resources owned by that variable are given back to the system. 
For instance, dropping a 'File' closes it. This is good, unused resources are 
reclaimed immediately when not needed. A Rust-specific issue is that a variable 
may appear to be in scope, but its value has moved.

A reference 'rs1' is made to a value 'tmp' which only lives for the duration of
its block: */

fn main() {
   let s1 = "hello dolly".to_string();
   let mut rs1 = &s1;
   {
       let tmp = "hello world".to_string();
       rs1 = &tmp;
   }
   println!("ref {}", rs1);
}

/* We borrow the value of 's1' and then borrow the value of 'tmp', but the 'tmp'
value does not exist outside that block. This saves you from the 'dangling 
pointer' problem of C - a reference that points to stale data.

error: `tmp` does not live long enough
  --> ref1.rs:8:5
   |
7  |         rs1 = &tmp;
   |                --- borrow occurs here
8  |     }
   |     ^ `tmp` dropped here while still borrowed
9  |     println!("ref {}", rs1);
10 | }
   | - borrowed value needs to live until here
*/