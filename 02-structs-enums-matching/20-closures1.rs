// closures1.rs

/* Closures are a powerful tool. In their simplest form, they act like shortcut functions. */

let f = |x| x * x;
let res = f(10);
println!("res {}", res);
// res 100

/* There are no explicit types in this example, everything is deduced, starting with the integer literal 10. We get an error if we call 'f' on different types since Rust has decided that 'f' must be called on an integer type.

    let res = f(10);

    let resf = f(1.2);
  |
8 |     let resf = f(1.2);
  |                  ^^^ expected integral variable, found floating-point variable
  |
  = note: expected type `{integer}`
  = note:    found type `{float}`
*/

// The first call fixes the type of the argument x, it's equivalent to:
fn f (x: i32) -> i32 {
    x * x
}

// There's a big difference between functions and closures besides explicit typing.

// Evaluate a linear function
let m = 2.0;
let c = 1.0;
let lin = |x| m*x + c;
println!("res {} {}", lin(1.0), lin(2.0));
// res 3 5

/* You cannot do this with the explicit 'fn' form, it does not know about variables in the enclosing scope. The closure has borrowed 'm' and 'c' from its context. Only 'rustc' knows the type of 'lin'. Under the hood, a closure is a struct that is callable, i.e. it implements the call operator. It behaves as if it was written out like so: */

struct MyAnonymousClosure1<'a> {
    m: &'a f64,
    c: &'a f64
}

impl <'a>MyAnonymousClosure1<'a> {
    fn call(&self, x: f64) -> f64 {
        self.m * x  + self.c
    }
}

/* Since a closure is a struct and it borrows values from its environment, it has a lifetime. All closures are unique types, but they have traits in common, so even though we don't know the exact type we know the generic constraint. */

// 'apply' works for any type 'T' as long as 'T' implements 'Fn(f64)->f64', i.e. is a function which takes 'f64' and returns 'f64'.
fn apply<F>(x: f64, f: F) -> f64
where F: Fn(f64)->f64  {
    f(x)
}
...
    let res1 = apply(3.0,lin);
    let res2 = apply(3.14, |x| x.sin());

/* After the call to 'apply(3.0,lin)', trying to access 'lin' gives an interesting error:

    let l = lin;
error[E0382]: use of moved value: `lin`
  --> closure2.rs:22:9
   |
16 |     let res = apply(3.0,lin);
   |                         --- value moved here
...
22 |     let l = lin;
   |         ^ value used here after move
   |
   = note: move occurs because `lin` has type
    `[closure@closure2.rs:12:15: 12:26 m:&f64, c:&f64]`,
     which does not implement the `Copy` trait

'apply' ate our closure, and there's the actual type of the struct that 'rustc' made up to implement it. Always thinking of closures as structs is helpful. Calling a closure is a method call. The three kinds of function traits correspond to the three kinds of methods:

'Fn' struct passed as '&self'
'FnMut' struct passed as '&mut self'
'FnOnce' struct passed as 'self'

So it's possible for a closure to mutate its captured references: */

    fn mutate<F>(mut f: F)
    where F: FnMut() {
        f()
    }
    // Note the 'mut', 'f' needs to be mutable for this to work
    let mut s = "world";
    mutate(|| s = "hello");
    assert_eq!(s, "hello");

// You cannot escape the rules for borrowing, consider this:
let mut s = "world";

// closure does a mutable borrow of s
let mut changer = || s = "world";

changer();
// does an immutable borrow of s
assert_eq!(s, "world");

/* This can't be done, the error is that we cannot borrow 's' in the assert statement because it has been previously borrowed by the closure 'changer' as mutable. As long as that closure lives, no other code can access s, so the solution is to control that lifetime by putting the closure in a limited scope. */

let mut s = "world";
{
    let mut changer = || s = "world";
    changer();
}
assert_eq!(s, "world");


/* You may wonder why closures in Rust are so complex compared to other languages. This is the cost of Rust's promise to not make any non-expicit allocations. In JavaScript, the equivalent 'mutate(function() {s = "hello";})' will always result in a dynamically allocated closure. Sometimes you don't want a closure to borrow  variables, but instead move them. */

let name = "dolly".to_string();
let age = 42;

let c = move || {
    println!("name {} age {}", name,age);
};

c();

println!("name {}",name);


/* The error at the last 'println' is "use of moved value: name". One solution here, if you want to keep name alive, is to move a cloned copy into the closure. */

let cname = name.to_string();
let c = move || {
    println!("name {} age {}",cname,age);
};

/* Moved closures are needed because we might need to call them at a point where the original context no longer exists, such as when creating a thread. A moved closure does not borrow so it does not have a lifetime. A major use of closures is within iterator methods. Recall the 'range' iterator we defined to go over a range of floating-point numbers. */

let sine: Vec<f64> = range(0.0,1.0,0.1).map(|x| x.sin()).collect();

// 'map' isn't defined on vectors, even though it's easy to create a trait that does this, because then every map will create a new vector.

// No temporary objects are created
let sum: f64 = range(0.0,1.0,0.1).map(|x| x.sin()).sum();

/* This will be as fast as writing out an explicit loop. That performance guarantee would be impossible if Rust closures were like Javascript closures. 'filter' is another useful iterator method; it only lets through values that match a condition. */

let tuples = [(10,"ten"),(20,"twenty"),(30,"thirty"),(40,"forty")];
let iter = tuples.iter().filter(|t| t.0 > 20).map(|t| t.1);

for name in iter {
    println!("{} ", name);
}
// thirty
// forty 