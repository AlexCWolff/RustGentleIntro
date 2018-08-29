// mod1.rs

/* As programs get larger, it's necessary to spread them over more than one file and put functions and types in different namespaces. The Rust solution for both of these is modules. The actual files can be named arbitrarily.

In Rust the full name would look like 'primitive::display::set_width', and after saying use 'primitive::display' you can then refer to it as 'display::set_width'. You can even say use 'primitive::display::set_width' and then just say 'set_width', but it's not a good idea. For this to work filenames must follow some simple rules.

A new keyword mod is used to define a module as a block where Rust types or functions can be written. */

mod foo {
    #[derive(Debug)]
    struct Foo {
        s: &'static str
    }
}

fn main() {
    let f = foo::Foo{s: "hello"};
    println!("{:?}", f);
}

/* It's still not quite right; we get 'struct Foo is private'. To solve this, we need the 'pub' keyword to export 'Foo'. The error then changes to "field s of struct foo::Foo is private", so put 'pub' before the field 's' to export 'Foo::s'. Needing an explicit 'pub' means that you must choose what items to make public from a module. The set of functions and types exported from a module is called its interface. */

pub struct Foo {
    pub s: &'static str
}