// cell.rs

// It's possible to get around the restrictions of the borrow checker

use std::cell::Cell;

fn main() {
    let answer = Cell::new(42);

    assert_eq!(answer.get(), 42);

    answer.set(77);

    assert_eq!(answer.get(), 77);
}
/* The answer was changed but the variable answer was not mutable. This is perfectly safe, since the value inside the cell is only accessed through 'set' and 'get'. This is called interior mutability. The usual is called inherited mutability: if I have a struct value 'v', then I can only write to a field 'v.a' if 'v' itself is writeable. Cell values relax this rule, since we can change the value contained within them with 'set' even if the cell itself is not mutable. However, 'Cell' only works with 'Copy' types, e.g primitive types and user types deriving the 'Copy' trait. */