// thread5.rs

/* Threads can't share the same environment by design in Rust. In particular, they cannot share regular references because the closures move their captured variables. shared references are fine however, because their lifetime is 'as long as needed', but you cannot use 'Rc' for this. This is because 'Rc' is not thread safe; it's optimized to be fast for the non-threaded case. Fortunately it is a compile error to use 'Rc' here.

For threads, you need 'std::sync::Arc'. 'Arc' stands for 'Atomic Reference Counting', that is, it guarantees that the reference count will be modified in one logical operation. To make this guarantee, it must ensure that the operation is locked so that only the current thread has access. clone is still much cheaper than actually making a copy however. */

// thread5.rs
use std::thread;
use std::sync::Arc;

struct MyString(String);

impl MyString {
    fn new(s: &str) -> MyString {
        MyString(s.to_string())
    }
}

fn main() {
    let mut threads = Vec::new();
    let name = Arc::new(MyString::new("dolly"));

    for i in 0..5 {
        let tname = name.clone();
        let t = thread::spawn(move || {
            println!("hello {} count {}", tname.0, i);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().expect("thread failed");
    }
}

/* We've deliberately created a wrapper type for 'String' here (a 'newtype') since our 'MyString' does not implement 'Clone'. But the shared reference can be cloned. The shared reference name is passed to each new thread by making a new reference with clone and moving it into the closure. It's a little verbose, but this is a safe pattern. Safety is important in concurrency precisely because the problems are so unpredictable. A program may run fine on your machine, but occasionally crash on the server. Worse still, the symptoms of such problems are not easy to diagnose. */