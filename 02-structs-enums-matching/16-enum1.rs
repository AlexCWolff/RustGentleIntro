// enum1.rs

// Enums are types which have a few definite values.

enum Direction {
    Up,
    Down,
    Left,
    Right
}
...
// `start` is type `Direction`
let start = Direction::Left;

// They can have methods defined on them, just like structs. The 'match' expression is the basic way to handle 'enum' values.

impl Direction {
    fn as_str(&self) -> &'static str {
        match *self { // *self has type Direction
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right"
        }
    }
}

/* Note that '*' before 'self'. It's easy to forget because often Rust will assume it (we said self.first_name, not (*self).first_name). However, matching is a more exact business. Leaving it out would give errors because of this type mismatch:

= note: expected type `&Direction`
= note:    found type `Direction`

This is because 'self' has type '&Direction', so we have to throw in the '*' to deference the type. Like structs, enums can implement traits, and '#[derive(Debug)]' can be added to 'Direction'.

println!("start {:?}",start);
// start Left

The 'as_str' method isn't necessary since we can always get the name from 'Debug', but as_str does not allocate, which may be important. You should not assume any particular ordering here, there's no implied integer 'ordinal' value. */

// A method which defines the successor of each 'Direction' value. The wildcard temporarily puts the enum names into the method context.
fn next(&self) -> Direction {
    use Direction::*;
    match *self {
        Up => Right,
        Right => Down,
        Down => Left,
        Left => Up
    }
}
...

let mut d = start;
for _ in 0..8 {
    println!("d {:?}", d);
    d = d.next();
}
// d Left
// d Up
// d Right
// d Down
// d Left
// d Up
// d Right
// d Down

/* This will cycle endlessly through the various directions in this particular order, it's a very simple state machine. 

// These enum values can't be compared.
assert_eq!(start, Direction::Left);

error[E0369]: binary operation `==` cannot be applied to type `Direction`
  --> enum1.rs:42:5
   |
42 |     assert_eq!(start, Direction::Left);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: an implementation of `std::cmp::PartialEq` might be missing for `Direction`
  --> enum1.rs:42:5

Instead say '#[derive(Debug,PartialEq)]' in front of enum 'Direction'.

Rust user-defined types start out blank, you give them sensible default behaviours by implementing the common traits. This also applies to structs; if you ask for Rust to derive 'PartialEq' for a struct it will do the sensible thing and assume that all fields implement it and build up a comparison. If this isn't so or you want to redefine equality, then you are free to define 'PartialEq' explicitly. */