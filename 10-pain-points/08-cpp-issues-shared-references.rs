/* Rust has smart pointers like C++ - for instance, the equivalent of std::unique_ptr is Box. There's no need for delete, since any memory or other resources will be reclaimed when the box goes out of scope (Rust very much embraces RAII).

let mut answer = Box::new("hello".to_string());
*answer = "world".to_string();
answer.push('!');
println!("{} {}", answer, answer.len());
People find to_string irritating at first, but it is explicit.

Note the explicit dererefence *, but methods on smart pointers don't need any special notation (we do not say (*answer).push('!'))

Obviously, borrowing only works if there is a clearly defined owner of the original content. In many designs this isn't possible.

In C++, this is where std::shared_ptr is used; copying just involves modifying a reference count on the common data. This is not without cost, however:

even if the data is read-only, constantly modifying the reference count can cause cache invalidation
std::shared_ptr is designed to be thread-safe and carries locking overhead as well
In Rust, std::rc::Rc also acts like a shared smart pointer using reference-counting. However, it is for immutable references only! If you want a thread-safe variant, use std::sync::Arc (for 'Atomic Rc'). So Rust is being a little awkward here in providing two variants, but you get to avoid the locking overhead for non-threaded operations.

These must be immutable references because that is fundamental to Rust's memory model. However, there's a get-out card: std::cell::RefCell. If you have a shared reference defined as Rc<RefCell<T>> then you can mutably borrow using its borrow_mut method. This applies the Rust borrowing rules dynamically - so e.g. any attempt to call borrow_mut when a borrow was already happening will cause a panic.

This is still safe. Panics will happen before any memory has been touched inappropriately! Like exceptions, they unroll the call stack. So it's an unfortunate word for such a structured process - it's an ordered withdrawal rather than a panicked retreat.

The full Rc<RefCell<T>> type is clumsy, but the application code isn't unpleasant. Here Rust (again) is prefering to be explicit.

If you wanted thread-safe access to shared state, then Arc<T> is the only safe way to go. If you need mutable access, then Arc<Mutex<T>> is the equivalent of Rc<RefCell<T>>. Mutex works a little differently than how it's usually defined: it is a container for a value. You get a lock on the value and can then modify it.

let answer = Arc::new(Mutex::new(10));

// in another thread
..
{
  let mut answer_ref = answer.lock().unwrap();
  *answer_ref = 42;
}
Why the unwrap? If the previous holding thread panicked, then this lock fails. (It's one place in the documentation where unwrap is considered a reasonable thing to do, since clearly things have gone seriously wrong. Panics can always be caught on threads.)

It's important (as always with mutexes) that this exclusive lock is held for as little time as possible. So it's common for them to happen in a limited scope - then the lock ends when the mutable reference goes out of scope.

Compared with the apparently simpler situation in C++ ("use shared_ptr dude") this seems awkward. But now any modifications of shared state become obvious, and the Mutex lock pattern forces thread safety.

Like everything, use shared references with caution. */