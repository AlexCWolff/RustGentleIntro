// thread6.rs (aka thread9.rs)

/* There are ways to send data between threads. This is done in Rust using channels. 'std::sync::mpsc::channel()' returns a tuple consisting of the receiver channel and the sender channel. Each thread is passed a copy of the sender with clone, and calls send. Meanwhile the main thread calls 'recv' on the receiver. 'MPSC' stands for 'Multiple Producer Single Consumer'. We create multiple threads which attempt to send to the channel, and the main thread 'consumes' the channel. */

// thread9.rs
use std::thread;
use std::sync::mpsc;

fn main() {
    let nthreads = 5;
    let (tx, rx) = mpsc::channel();

    for i in 0..nthreads {
        let tx = tx.clone();
        thread::spawn(move || {
            let response = format!("hello {}", i);
            tx.send(response).unwrap();
        });
    }

    for _ in 0..nthreads {
        println!("got {:?}", rx.recv());
    }
}
// got Ok("hello 0")
// got Ok("hello 1")
// got Ok("hello 3")
// got Ok("hello 4")
// got Ok("hello 2")

/* There's no need to join here since the threads send their response just before they end execution, but obviously this can happen at any time. 'recv' will block, and will return an error if the sender channel is disconnected. 'recv_timeout' will only block for a given time period, and may return a timeout error as well. 'send' never blocks, which is useful because threads can push out data without waiting for the receiver to process. In addition, the channel is buffered so multiple 'send' operations can take place, which will be received in order. However, not blocking means that 'Ok' does not automatically mean successfully delivered message. A 'sync_channel' does block on 'send'. With an argument of zero, the 'send' blocks until the 'recv' happens. The threads must meet up or rendezvous. */

let (tx, rx) = mpsc::sync_channel(0);

let t1 = thread::spawn(move || {
    for i in 0..5 {
        tx.send(i).unwrap();
    }
});

for _ in 0..5 {
    let res = rx.recv().unwrap();
    println!("{}",res);
}
t1.join().unwrap();

/* We can easily cause an error here by calling 'recv' when there has been no corresponding 'send', e.g by looping 'for i in 0..4'. The thread ends, and 'tx' drops, and then 'recv' will fail. This will also happen if the thread panics, which causes its stack to be unwound, dropping any values. If the 'sync_channel' was created with a non-zero argument 'n', then it acts like a queue with a maximum size of 'n'; 'send' will only block when it tries to add more than 'n' values to the queue. Channels are strongly typed, here the channel had type 'i32', but type inference makes this implicit. If you need to pass different kinds of data, then enums are a good way to express this. */