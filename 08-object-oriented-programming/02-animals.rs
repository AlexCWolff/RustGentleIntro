/* Any discussion of OOP and inheritance ends up using animals as an example. It makes for a nice story: "See, a Cat is a Carnivore. And a Carnivore is an Animal". The classic slogan from Ruby and Python is "if it quacks, it's a duck". All your objects have to do is define 'quack' and they can be considered to be ducks, albeit in a very narrow way. */

trait Quack {
    fn quack(&self);
}

struct Duck ();

impl Quack for Duck {
    fn quack(&self) {
        println!("quack!");
    }
}

struct RandomBird {
    is_a_parrot: bool
}

impl Quack for RandomBird {
    fn quack(&self) {
        if ! self.is_a_parrot {
            println!("quack!");
        } else {
            println!("squawk!");
        }
    }
}

let duck1 = Duck();
let duck2 = RandomBird{is_a_parrot: false};
let parrot = RandomBird{is_a_parrot: true};

let ducks: Vec<&Quack> = vec![&duck1,&duck2,&parrot];

for d in &ducks {
    d.quack();
}
// quack!
// quack!
// squawk!

/* Here we have two completely different types, one of which doesn't even have data, and they both 'quack()'. One is behaving a little odd for a duck but they share the same method name and Rust can keep a collection of such objects in a type-safe way. Type safety is a fantastic thing, without static typing you could insert a cat into that collection of Quackers, resulting in run-time chaos. */

impl Quack for i32 {
    fn quack(&self) {
        for i in 0..*self {
            print!("quack {} ",i);
        }
        println!("");
    }
}

let int = 4;

let ducks: Vec<&Quack> = vec![&duck1,&duck2,&parrot,&int];
...
// quack!
// quack!
// squawk!
// quack 0 quack 1 quack 2 quack 3

/* It quacks so it must be a duck. You can apply your traits to any Rust value, not just 'objects'. Since 'quack' is passed a reference, there's an explicit dereference '*' to get the integer. However, you can only do this with a trait and a type from the same crate, so the standard library cannot be 'monkey patched', which is a not wildly admired practice of Ruby. Up to this point the trait 'Quack' was behaving like a Java interface, and like modern Java interfaces you can have provided methods which supply a default implementation if you have implemented the required methods (the 'Iterator' trait is a good example). But traits are not part of the definition of a type and you can define and implement new traits on any type, subject to the same-crate restriction. */

// Pass a reference to any Quack implementor
fn quack_ref (q: &Quack) {
    q.quack();
}

quack_ref(&d);

/* That's Rust-style subtyping. Go has an interesting take on this; if there's a Go interface 'Quack', and a type has a 'quack' method, then that type satisfies 'Quack' without any need for explicit definition. This also breaks the baked-into-definition Java model and allows compile-time duck-typing, at the cost of some clarity and type-safety. There is a problem with duck-typing; one of the signs of bad OOP is too many methods which have some generic name like 'run'. It is possible for a Go interface to be accidentally valid. In Rust both the 'Debug' and 'Display' traits define 'fmt' methods, but they really mean different things.

Rust traits allow traditional polymorphic OOP, but what about inheritance? People usually mean implementation inheritance whereas Rust does interface inheritance. It's as if a Java programmer never used 'extend' and instead used 'implements'. This is actually recommended practice, even in Java you've probably been overdoing classes.

Implementation inheritance has some serious problems, but it does feel very convenient. There's this huge base class called 'Animal' and it has lots of useful functionality which our derived class 'Cat' can use. It's a form of code reuse, but code reuse is a separate concern. Getting the distinction between implementation and interface inheritance is important when understanding Rust.

Traits may have provided methods. Consider 'Iterator'; you only have to override 'next', but get a whole host of methods free. This is similar to 'default' methods of modern Java interfaces. Here we only define 'name' and 'upper_case' is defined for us. We could override 'upper_case' as well, but it isn't required. */

trait Named {
    fn name(&self) -> String;

    fn upper_case(&self) -> String {
        self.name().to_uppercase()
    }
}

struct Boo();

impl Named for Boo {
    fn name(&self) -> String {
        "boo".to_string()
    }
}

let f = Boo();

assert_eq!(f.name(),"boo".to_string());
assert_eq!(f.upper_case(),"BOO".to_string());
// This is a kind of code reuse, but note that it does not apply to the data, only the interface