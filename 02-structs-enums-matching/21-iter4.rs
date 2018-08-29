// iter4.rs

// The three kinds correspond (again) to the three basic argument types.

// explicit
for s in vec.iter() {...} // &String
for s in vec.iter_mut() {...} // &mut String
for s in vec.into_iter() {...} // String

// implicit!
for s in &vec {...} // &String
for s in &mut vec {...} // &mut String
for s in vec {...} // String

/* Prefer being explicit, but it's important to understand both forms and their implications. 'into_iter' consumes the vector and extracts its strings, and so afterwards the vector is no longer available. The implicit form 'for s in &vec' is usually the one you want, just as '&T' is a good default in passing arguments to functions.

Rust relies heavily on type deduction, you won't often see explicit types in closure arguments, so it's important to understand how the three kinds work. This saves typing but the price of this compact code is that you need to know what the implicit types actually are.

'map' takes whatever value the iterator returns and converts it into something else, but 'filter' takes a reference to that value. In this case, we're using 'iter' so the iterator item type is '&String'. Note that 'filter' receives a reference to this type. */

for n in vec.iter().map(|x: &String| x.len()) {...} // n is usize
....
}

for s in vec.iter().filter(|x: &&String| x.len() > 2) { // s is &String
...
}

/* When calling methods Rust will dereference automatically, so the problem isn't obvious, but '|x: &&String| x == "one"|' will not work because operators are more strict about type matching. The error is that there is no such operator that compares '&&String' and '&str'. So you need an explicit deference to make that '&&String' into a '&String' which does match. */

for s in vec.iter().filter(|x: &&String| *x == "one") {...}
// same as implicit form
for s in vec.iter().filter(|x| *x == "one") {...}

// If you leave out the explicit type you can modify the argument so that the type of 's' is now '&String', this is usually how you will see it written.
for s in vec.iter().filter(|&x| x == "one")