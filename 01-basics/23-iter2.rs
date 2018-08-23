// iter2.rs

// Seems innefficient, but when compiled it's just as fast as a while loop.
fn main() {
    let arr = [10, 20, 30];
    for i in arr {
        println!("{}", i);
    }
}

/* Oops
4 |     for i in arr {
  |     ^ the trait `std::iter::Iterator` is not implemented for `[{integer}; 3]`
  |
  = note: `[{integer}; 3]` is not an iterator; maybe try calling
   `.iter()` or a similar method
  = note: required by `std::iter::IntoIterator::into_iter` */