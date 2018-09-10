/* A common problem with object-oriented design is trying to force things into a is-a relationship and neglecting the has-a relationships. You should always prefer composition to inheritance, the Gang of Four says as much in their famous Design Patterns book twenty-two years ago.

Here's an example: you want to model the employees of some company, and 'Employee' seems a good name for a class. Then, Manager is-a Employee so we start building our hierarchy with a 'Manager' subclass of 'Employee'. This isn't as smart as it seems. Maybe we got carried away with identifying important nouns, maybe we unconsciously think that managers and employees are different kinds of animals. It's better if 'Employee' has-a 'Roles' collection, and then a manager is just an 'Employee' with more responsibilities and capabilities.

Consider 'Vehicles', ranging from bicycles to 300t trucks. There are multiple ways to think about vehicles: road-worthiness (all-terrain, city, rail-bound, etc), power-source (electric, diesel, diesel-electric, etc), cargo or passenger, etc. Any fixed hierarchy of classes you create based on one aspect ignores all other aspects, but there are multiple possible classifications.

Composition is very important in Rust for the obvious reason that you can't inherit functionality in a lazy way from a base class. Composition is also important because the borrow checker is smart enough to know that borrowing different struct fields are separate borrows. You can have a mutable borrow of one field while having an immutable borrow of another field, and so forth. Rust cannot tell that a method only accesses one field, so the fields should be structs with their own methods for ease of implementation. The external interface of the struct can be anything you like using suitable traits. */

// A concrete example of 'split borrrowing'
// We have a struct that owns some strings, with a method for borrowing the first string mutably.
struct Foo {
    one: String,
    two: String
}

impl Foo {
    fn borrow_one_mut(&mut self) -> &mut String {
        &mut self.one
    }
    ....
}
// This is an example of a Rust naming convention ; such methods should end in '_mut'

// A method for borrowing both strings, reusing the first method
fn borrow_both(&self) -> (&str,&str) {
    (self.borrow_one_mut(), &self.two)
}

// This can't work, we've borrrowed mutably from 'self' and also borrowed immmutably from 'self'.
// If Rust allowed situations like this then that immmutable reference can't be guaranteed not to change.

// The solution is simple
fn borrow_both(&self) -> (&str,&str) {
    (&self.one, &self.two)
}

/* This is fine because the borrow checker considers these to be independent borrows. Imagine that the fields were some arbitrary types, and you can see that methods called on these fields will not cause borrowing problems. There is a restricted but very important kind of "inheritance" with 'Deref', which is the trait for '*'. 'String' implements 'Deref<Target=str>' and so all the methods defined on '&str' are automatically available for 'String' as well. In a similar way, the methods of Foo can be directly called on 'Box<Foo>'. It really should be used for cases where there is an owned, mutable type and a simpler borrowed type. */

// Generally in Rust there is trait inheritance
trait Show {
    fn show(&self) -> String;
}

trait Location {
    fn location(&self) -> String;
}

trait ShowTell: Show + Location {}

// The last trait simply combines our two distinct traits into one, although it could specify other methods.

#[derive(Debug)]
struct Foo {
    name: String,
    location: String
}

impl Foo {
    fn new(name: &str, location: &str) -> Foo {
        Foo{
            name: name.to_string(),
            location: location.to_string()
        }
    }
}

impl Show for Foo {
    fn show(&self) -> String {
        self.name.clone()
    }
}

impl Location for Foo {
    fn location(&self) -> String {
        self.location.clone()
    }
}

impl ShowTell for Foo {}

// Now if we have a value foo of type 'Foo', then a reference to that value will satisfy '&Show', '&Location', or '&ShowTell' which implies both.

// A useful macro
macro_rules! dbg {
    // Takes one argument, represented by $x which must be an 'expression'
    ($x:expr) => {
        // Print out its value and a stringified version of the value
        println!("{} = {:?}",stringify!($x),$x);
    }
}
// This means that if I passed '1+2' then 'stringify!(1+2)' is the literal string "1+2". This will save us some typing when playing with code.

let foo = Foo::new("Pete","bathroom");
dbg!(foo.show());
dbg!(foo.location());

let st: &ShowTell = &foo;

dbg!(st.show());
dbg!(st.location());

fn show_it_all(r: &ShowTell) {
    dbg!(r.show());
    dbg!(r.location());
}

let boo = Foo::new("Alice","cupboard");
show_it_all(&boo);

fn show(s: &Show) {
    dbg!(s.show());
}

show(&boo);

// foo.show() = "Pete"
// foo.location() = "bathroom"
// st.show() = "Pete"
// st.location() = "bathroom"
// r.show() = "Alice"
// r.location() = "cupboard"
// s.show() = "Alice"

/* This is object-orientation, just not the kind you may be used to. Note that the 'Show' reference passed to 'show' can not be dynamically upgraded to a 'ShowTell'. Languages with more dynamic class systems allow you to check whether a given object is an instance of a class and then to do a dynamic cast to that type. It isn't a good idea in general and cannot work in Rust because that 'Show' reference has "forgotten" that it was originally a 'ShowTell' reference.

You always have a choice: polymorphic via trait objects, or monomorphic via generics constrainted by traits. Modern C++ and the Rust standard library tends to take the generic route, but the polymorphic route is not obsolete. You do have to understand the different trade-offs; generics generate the fastest code, which can be inlined. This may lead to code bloat, but not everything needs to be as fast as possible; it may only happen a 'few' times in the lifetime of a typical program run.

In summary:
* the role played by class is shared between data and traits
* structs and enums are dumb, although you can define methods and do data hiding
* a limited form of subtyping is possible on data using the Deref trait
* traits don't have any data, but can be implemented for any type (not just structs)
* traits can inherit from other traits
* traits can have provided methods, allowing interface code re-use
* traits give you both virtual methods (polymorphism) and generic constraints (monomorphism) */