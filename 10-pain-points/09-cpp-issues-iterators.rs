/* Iterators in C++ are defined fairly informally; they involve smart pointers, usually starting with c.begin() and ending with c.end(). Operations on iterators are then implemented as stand-alone template functions, like std::find_if.

Rust iterators are defined by the Iterator trait; next returns an Option and when the Option is None we are finished.

The most common operations are now methods. Here is the equivalent of find_if. It returns an Option (case of not finding is None) and here the if let statement is convenient for extracting the non-None case:

let arr = [10, 2, 30, 5];
if let Some(res) = arr.find(|x| x == 2) {
    // res is 2
} */