// refcell.rs

/* For other values, we have to get a reference we can work on, either mutable or immutable. This is what 'RefCell' provides; you ask it explicitly for a reference to the contained value. */

// refcell.rs
use std::cell::RefCell;

fn main() {
    let greeting = RefCell::new("hello".to_string());

    assert_eq!(*greeting.borrow(), "hello");
    assert_eq!(greeting.borrow().len(), 5);

    *greeting.borrow_mut() = "hola".to_string();

    assert_eq!(*greeting.borrow(), "hola");
}
/* Again, greeting was not declared as mutable. The explicit dereference operator '*' can be a bit confusing in Rust, because often you don't need it; for instance 'greeting.borrow().len()' is fine since method calls will dereference implicitly. But you do need '*' to pull out the underlying '&String' from 'greeting.borrow()' or the '&mut' 'String' from 'greeting.borrow_mut()'. Using a 'RefCell' isn't always safe, because any references returned from these methods must follow the usual rules. */ 

let mut gr = greeting.borrow_mut(); // gr is a mutable borrow
*gr = "hola".to_string();

assert_eq!(*greeting.borrow(), "hola"); // <== we blow up here
// thread 'main' panicked at 'already mutably borrowed: BorrowError'

/* You cannot borrow immutably if you have already borrowed mutably. Except, and this is important, the violation of the rules happens at runtime. The solution, as always, is to keep the scope of mutable borrows as limited as possible; in this case, you could put a block around the first two lines here so that the mutable reference 'gr' gets dropped before we borrow again. This is not a feature you use without good reason, since you will not get a compile-time error. These types provide dynamic borrowing in cases where the usual rules make some things impossible. */