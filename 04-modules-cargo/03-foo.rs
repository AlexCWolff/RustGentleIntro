// foo.rs

/* Everyone gets to a point where they want to break a program up into separate files, depending on personal taste. To break this program into separate files, we put the foo code into 'foo.rs'  and use a 'mod foo' statement without a block in the main program to use the foo code. */

#[derive(Debug)]
pub struct Foo {
    s: &'static str
}

impl Foo {
    pub fn new(s: &'static str) -> Foo {
        Foo{s: s}
    }
}

