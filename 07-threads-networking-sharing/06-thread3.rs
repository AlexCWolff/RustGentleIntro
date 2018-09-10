// thread3.rs (aka thread4.rs)

/* The returned objects can be used to keep track of multiple threads:

// thread4.rs
use std::thread;

fn main() {
    let mut threads = Vec::new();

    for i in 0..5 {
        let t = thread::spawn(move || {
            println!("hello {}", i);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().expect("thread failed");
    }
}
// hello 0
// hello 2
// hello 4
// hello 3
// hello 1

Rust insists that we handle the case where the join failed - i.e. that thread panicked. (You would typically not bail out of the main program when this happens, just note the error, retry etc)

There is no particular order to thread execution (this program gives different orders for different runs), and this is key - they really are independent threads of execution. Multithreading is easy; what's hard is concurrency - managing and synchronizing multiple threads of execution. */