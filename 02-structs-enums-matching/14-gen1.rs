// gen1.rs

/* Here is a generic function which will dump out any value that implements 'Debug'. Since it's generic we can pass a reference to any type of value. 'T'' is a type parameter, which needs to be declared just after the function name. */

fn dump<T> (value: &T) {
    println!("value is {:?}",value);
}

let n = 42;
dump(&n);

/* 
error[E0277]: the trait bound `T: std::fmt::Debug` is not satisfied
...
   = help: the trait `std::fmt::Debug` is not implemented for `T`
   = help: consider adding a `where T: std::fmt::Debug` bound

Rust knows nothing about this generic type 'T' and needs to be told that 'T' implements 'Debug' */

fn dump<T> (value: &T)
where T: std::fmt::Debug {
    println!("value is {:?}",value);
}

let n = 42;
dump(&n);
// value is 42

/* Rust generic functions need trait bounds on types; we are saying here that "T is any type that implements Debug". 'rustc' suggests exactly what bound needs to be provided. Now that Rust knows the trait bounds for 'T' it can give you sensible compiler messages */

struct Foo {
    name: String
}

let foo = Foo{name: "hello".to_string()};

dump(&foo)

/* 
Error: the trait 'std::fmt::Debug' is not implemented for Foo.

Functions are already generic in dynamic languages because values carry their actual type around and the type checking happens at run-time or fails. For larger programs we want to know about problems at compile-time. Murphy's Law implies that run-time problems will tend to happen when most inconvenient.

The operation of squaring a number is generic: 'x*x' will work for integers, floats and generally for anything that knows about the multiplication operator '*'. But what are the type bounds? */

fn sqr<T> (x: T) -> T {
    x * x
}

fn main() {
    let res = sqr(10.0);
    println!("res {}",res);
}

/*
error[E0369]: binary operation `*` cannot be applied to type `T`
 --> gen1.rs:4:5
  |
4 |     x * x
  |     ^
  |
note: an implementation of `std::ops::Mul` might be missing for `T`
 --> gen1.rs:4:5
  |
4 |     x * x
  |     ^

Rust does not know that 'T' can be multiplied. Following the advice of the compiler, let's constrain that type parameter using that trait, which is used to implement the multiplication operator '*'. */

fn sqr<T> (x: T) -> T
where T: std::ops::Mul {
    x * x
}

/*
error[E0308]: mismatched types
 --> gen2.rs:6:5
  |
6 |     x * x
  |     ^^^ expected type parameter, found associated type
  |
  = note: expected type `T`
  = note:    found type `<T as std::ops::Mul>::Output`

The type of 'x*x' is the associated type 'T::Output', not 'T'. Mathematically there's no reason that the type of 'x*x' would be the same as the type of 'x', e.g. the dot product of two vectors is a scalar. */

fn sqr<T> (x: T) -> T::Output
where T: std::ops::Mul {
    x * x
}

/* 
error[E0382]: use of moved value: `x`
 --> gen2.rs:6:7
  |
6 |     x * x
  |     - ^ value used here after move
  |     |
  |     value moved here
  |
  = note: move occurs because `x` has type `T`, which does not implement the `Copy` trait
*/