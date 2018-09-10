// thread1.rs

/* Over the last twenty years, there has been a shift away from raw processing speed to CPUs having multiple cores. So the only way to get the most out of a modern computer is to keep all of those cores busy. It's certainly possible to spawn child processes in the background as we saw with 'Command' but there's still a synchronization problem: we don't know exactly when those children are finished without waiting on them. There are other reasons for needing separate threads of execution. You can't afford to lock up your whole process just to wait on blocking i/o, for instance. Spawning threads is straightforward in Rust, you feed spawn a closure which is executed in the background. */

// thread1.rs
use std::thread;
use std::time;

fn main() {
    thread::spawn(|| println!("hello"));
    thread::spawn(|| println!("dolly"));

    println!("so fine");
    // wait a little bit
    thread::sleep(time::Duration::from_millis(100));
}
// so fine
// hello
// dolly */