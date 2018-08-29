// mod2.rs

// It is usually better to hide the insides of a struct and only allow access through methods.
mod foo {
    #[derive(Debug)]
    pub struct Foo {
        s: &'static str
    }

    impl Foo {
        pub fn new(s: &'static str) -> Foo {
            Foo{s: s}
        }
    }
}

fn main() {
    let f = foo::Foo::new("hello");
    println!("{:?}", f);
}

/* Hiding the implementation is a good thing because you can change it later without breaking the interface, without consumers of a module getting too dependent on its details. The enemy of large-scale programing is a tendency for code to get entangled so that understanding a piece of code is impossible in isolation. In a perfect world a module does one thing, does it well, and keeps its own secrets. However you shouldn't hide implementation when the interface is the implementation, like 'struct Point{x: f32, y: f32}'. Within a module all items are visible to each other. */