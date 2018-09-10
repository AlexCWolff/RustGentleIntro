// thread7.rs

/* Let's look at synchronization. 'join' is very basic, and merely waits until a particular thread has finished. A 'sync_channel' synchronizes two threads; in the last example, the spawned thread and the main thread are completely locked together. Barrier synchronization is a checkpoint where the threads must wait until all of them have reached that point. The barrier is created with the number of threads that we want to wait for. As before we use use 'Arc' to share the barrier with all the threads. */

// thread7.rs
use std::thread;
use std::sync::Arc;
use std::sync::Barrier;

fn main() {
    let nthreads = 5;
    let mut threads = Vec::new();
    let barrier = Arc::new(Barrier::new(nthreads));

    for i in 0..nthreads {
        let barrier = barrier.clone();
        let t = thread::spawn(move || {
            println!("before wait {}", i);
            barrier.wait();
            println!("after wait {}", i);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }
}
// before wait 2
// before wait 0
// before wait 1
// before wait 3
// before wait 4
// after wait 4
// after wait 2
// after wait 3
// after wait 0
// after wait 1

// The threads do their thing, all meet up, and then continue. It's like a resumable 'join' and useful when you need to split pieces of a job to different threads and want to take some action when all the pieces are finished.