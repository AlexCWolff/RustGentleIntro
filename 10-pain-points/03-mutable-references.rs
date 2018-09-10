/* The rule is: only one mutable reference at a time. The reason is that tracking mutability is hard when it can happen all over the place. Not obvious in dinky little programs, but things can get bad in big codebases.

The further constraint is that you can't have immutable references while there's a mutable reference out. Otherwise, anybody who has those references doesn't have a guarantee that they won't change. C++ also has immutable references (e.g. const string&) but does not give you this guarantee that someone can't keep a string& reference and modify it behind your back.

This is a challenge if you are used to languages where every reference is mutable! Unsafe, 'relaxed' languages depend on people understanding their own programs and nobly deciding not to do Bad Things. But big programs are written by more than one person and are beyond the power of a single individual to understand in detail.

The irritating thing is that the borrow checker is not as smart as it could be.

let mut m = HashMap::new();
m.insert("one", 1);
m.insert("two", 2);

if let Some(r) = m.get_mut("one") { // <-- mutable borrow of m
    *r = 10;
} else {
    m.insert("one", 1); // can't borrow mutably again!
}
Clearly this does not really violate the Rules since if we got None we haven't actually borrowed anything from the map.

There are various ugly workarounds:

let mut found = false;
if let Some(r) = m.get_mut("one") {
    *r = 10;
    found = true;
}
if ! found {
    m.insert("one", 1);
}
Which is yucky, but it works because the bothersome borrow is kept to the first if-statement.

The better way here is to use HashMap's entry API.

use std::collections::hash_map::Entry;

match m.entry("one") {
    Entry::Occupied(e) => {
        *e.into_mut() = 10;
    },
    Entry::Vacant(e) => {
        e.insert(1);
    }
};
The borrow checker will get less frustrating when non-lexical lifetimes arrive sometime this year.

The borrow checker does understand some important cases, however. If you have a struct, fields can be independently borrowed. So composition is your friend; a big struct should contain smaller structs, which have their own methods. Defining all the mutable methods on the big struct will lead to a situation where you can't modify things, even though the methods might only refer to one field.

With mutable data, there are special methods for treating parts of the data independently. For instance, if you have a mutable slice, then split_at_mut will split this into two mutable slices. This is perfectly safe, since Rust knows that the slices do not overlap. */