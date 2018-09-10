// thread2.rs

// Obviously just wait is not a rigorous solution. It's better to call 'join' on the returned object, then the main thread waits for the spawned thread to finish.

// thread2.rs
use std::thread;

fn main() {
    let t = thread::spawn(|| {
        println!("hello");
    });
    println!("wait {:?}", t.join());
}
// hello
// wait Ok(())
// Here's an interesting variation: force the new thread to panic.

let t = thread::spawn(|| {
    println!("hello");
    panic!("I give up!");
});
println!("wait {:?}", t.join());

/* We get a panic as expected, but only the panicking thread dies. We still manage to print out the error message from the 'join'. Panics are not always fatal, but threads are relatively expensive, so this should not be seen as a routine way of handling panics.

hello
thread '<unnamed>' panicked at 'I give up!', thread2.rs:7
note: Run with `RUST_BACKTRACE=1` for a backtrace.
wait Err(Any) 
*/