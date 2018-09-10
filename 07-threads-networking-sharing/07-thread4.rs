// thread4.rs (aka thread3.rs)

// It's possible for the thread closure to capture values, but by moving, not by borrowing.

// thread3.rs
use std::thread;

fn main() {
    let name = "dolly".to_string();
    let t = thread::spawn(|| {
        println!("hello {}", name);
    });
    println!("wait {:?}", t.join());
}

/*
error[E0373]: closure may outlive the current function, but it borrows `name`, which is owned by the current function
 --> thread3.rs:6:27
  |
6 |     let t = thread::spawn(|| {
  |                           ^^ may outlive borrowed value `name`
7 |         println!("hello {}", name);
  |                             ---- `name` is borrowed here
  |
help: to force the closure to take ownership of `name` (and any other referenced variables), use the `move` keyword, as shown:
  |     let t = thread::spawn(move || {

Imagine spawning this thread from a function, it will exist after the function call has finished and name gets dropped. Adding 'move' solves our problem. But this is a move, so 'name' may only appear in one thread. It is possible to share references, but they need to have static lifetime. */

let name = "dolly";
let t1 = thread::spawn(move || {
    println!("hello {}", name);
});
let t2 = thread::spawn(move || {
    println!("goodbye {}", name);
});
// 'name' exists for the whole duration of the program (static), so 'rustc' is satisfied that the closure will never outlive 'name'. However, most interesting references do not have static lifetimes.