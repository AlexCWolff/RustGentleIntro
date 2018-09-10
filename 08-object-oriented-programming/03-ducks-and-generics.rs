// An example of generic-friendly duck function in Rust would be this trivial one:
fn quack<Q> (q: &Q)
where Q: Quack {
    q.quack();
}

let d = Duck();
quack(&d);

/* The type parameter is any type which implements 'Quack'. There's an important difference between quack and the quack_ref defined in the last section. The body of this function is compiled for each of the calling types and no virtual method is needed; such functions can be completely inlined. It uses the trait 'Quack' in a different way, as a constraint on generic types. This is the C++ equivalent to the generic quack (note the const and that the type parameter is not constrained in any way):

template <class Q>
void quack(const Q& q) {
    q.quack();
}

This is very much compile-time duck-typing; if we pass a reference to a non-quackable type, then the compiler will complain about no 'quack' method. At least the error is found at compile-time, but it's worse when a type is accidentally Quackable, as happens with Go. More involved template functions and classes lead to terrible error messages, because there are no constraints on the generic types. 

// A function which could handle an iteration over Quacker pointers
template <class It>
void quack_everyone (It start, It finish) {
    for (It i = start; i != finish; i++) {
        (*i)->quack();
    }
}

This would then be implemented for each iterator type 'It'.
*/

//  The Rust equivalent, a little more challenging
fn quack_everyone <I> (iter: I)
where I: Iterator<Item=Box<Quack>> {
    for d in iter {
        d.quack();
    }
}

let ducks: Vec<Box<Quack>> = vec![Box::new(duck1),Box::new(duck2),Box::new(parrot),Box::new(int)];

quack_everyone(ducks.into_iter());

/* Iterators in Rust aren't duck-typed but are types that must implement 'Iterator', and in this case the iterator provides boxes of 'Quack'. There's no ambiguity about the types involved, and the values must satisfy 'Quack'. Often the function signature is the most challenging thing about a generic Rust function, so I recommend reading the source of the standard library. The implementation is often much simpler than the declaration. Here the only type parameter is the actual iterator type, which means that this will work with anything that can deliver a sequence of 'Box<Duck>', not just a vector iterator. */