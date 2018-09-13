/* Remember, only one mutable reference at a time. Tracking mutability is hard when it can happen all over the place. You also can't have immutable references while there's a mutable reference out. Otherwise, anybody who has those references doesn't have a guarantee that they won't change. C++ also has immutable references, 'const string&', but this does not give you this guarantee that someone can't keep a 'string&' reference and modify it behind your back. This is a challenge if you are used to languages where every reference is mutable. */

// The borrow checker is not always as smart as it could be
let mut m = HashMap::new();
m.insert("one", 1);
m.insert("two", 2);

if let Some(r) = m.get_mut("one") { // <-- mutable borrow of m
    *r = 10;
} else {
    m.insert("one", 1); // can't borrow mutably again
}
// Clearly this does not really violate the rules since if we got 'None' we haven't actually borrowed anything from the map.

// There are various ugly workarounds
let mut found = false;
if let Some(r) = m.get_mut("one") {
    *r = 10;
    found = true;
}
if ! found {
    m.insert("one", 1);
}
// It works because the bothersome borrow is kept to the first if-statement.

// The better way here is to use 'HashMap''s entry API
use std::collections::hash_map::Entry;

match m.entry("one") {
    Entry::Occupied(e) => {
        *e.into_mut() = 10;
    },
    Entry::Vacant(e) => {
        e.insert(1);
    }
};

/* The borrow checker will get less frustrating when non-lexical lifetimes arrive. If you have a struct, fields can be independently borrowed, so composition is your friend; a big struct should contain smaller structs, which have their own methods. Defining all the mutable methods on the big struct will lead to a situation where you can't modify things, even though the methods might only refer to one field. With mutable data, there are special methods for treating parts of the data independently. For instance, if you have a mutable slice, then 'split_at_mut' will split this into two mutable slices. This is perfectly safe, since Rust knows that the slices do not overlap. */