// thread8.rs (aka thread9.rs)

/* How can threads modify shared state? Recall the 'Rc<RefCell<T>>' strategy for dynamically doing a mutable borrow on shared references. The threading equivalent to 'RefCell' is 'Mutex'; you may get your mutable reference by calling lock. While this reference exists, no other thread can access it. 'mutex' stands for 'Mutual Exclusion'; we lock a section of code so that only one thread can access it, and then unlock it. You get the lock with the lock method, and it is unlocked when the reference is dropped. */

// thread9.rs
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let answer = Arc::new(Mutex::new(42));

    let answer_ref = answer.clone();
    let t = thread::spawn(move || {
        let mut answer = answer_ref.lock().unwrap();
        *answer = 55;
    });

    t.join().unwrap();

    let ar = answer.lock().unwrap();
    assert_eq!(*ar, 55);

}

/* This isn't so straightforward as using 'RefCell' because asking for the lock on the mutex might fail, if another thread has panicked while holding the lock. In this case, the documentation actually recommends just exiting the thread with 'unwrap' because things have gone very wrong. It's even more important to keep this mutable borrow as short as possible, because as long as the mutex is locked, other threads are blocked. This is not the place for expensive calculations. Typically such code would be used like this: */

// ... do something in the thread
// get a locked reference and use it briefly!
{
    let mut data = data_ref.lock().unwrap();
    // modify data
}
//... continue with the thread 