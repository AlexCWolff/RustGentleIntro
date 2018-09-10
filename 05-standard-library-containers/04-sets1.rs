// sets1.rs

/* Sets are maps where you care only about the keys, not any associated values. 'insert' only takes one value, and you use 'contains' for testing whether a value is in a set. Like all containers, you can create a 'HashSet' from an iterator, this is exactly what collect does once you have given it the necessary type hint. */

use std::collections::HashSet;

fn make_set(words: &str) -> HashSet<&str> {
    words.split_whitespace().collect()
}

fn main() {
    let fruit = make_set("apple orange pear orange");

    println!("{:?}", fruit);
}
// {"orange", "pear", "apple"}

// Note that repeated insertions of the same key has no effect, and the order of values in a set are not important.

// Standard set operations
let fruit = make_set("apple orange pear");
let colours = make_set("brown purple orange yellow");

for c in fruit.intersection(&colours) {
    println!("{:?}",c);
}
// "orange"

// They all create iterators, and you can use 'collect' to make these into sets.

// A shortcut, just as we defined for vectors
use std::hash::Hash;

trait ToSet<T> {
    fn to_set(self) -> HashSet<T>;
}

impl <T,I> ToSet<T> for I
where T: Eq + Hash, I: Iterator<Item=T> {

    fn to_set(self) -> HashSet<T> {
       self.collect()
    }
}

...

let intersect = fruit.intersection(&colours).to_set();

/* As with all Rust generics, you need to constrain types. This can only be implemented for types that understand equality ('Eq') and for which a hash function exists ('Hash'). Remember that there is no type called 'Iterator', so 'I' represents any type that implements 'Iterator'. 

There are rules for implementing our own methods on standard library types. We can only do this for our own traits. If both the struct and the trait came from the same crate (particularly, the stdlib) then such implemention would not be allowed. This prevents confusion, but you should be aware of the consequences. If 'make_set' was written so that these are sets of owned strings, then the actual type of 'intersect'woulb be 'HashSet<&String>'. */

fn make_set(words: &str) -> HashSet<String> {
    words.split_whitespace().map(|s| s.to_string()).collect()
}
...
// intersect is HashSet<&String>!
let intersect = fruit.intersection(&colours).to_set();


/* Rust will not suddenly start making copies of owned strings. 'intersect' contains a single '&String' borrowed from 'fruit'.This will probably cause you trouble later when you start patching up lifetimes. A better solution is to use the iterator's 'cloned' method to make owned string copies of the intersection. */

// intersect is HashSet<String> - much better
let intersect = fruit.intersection(&colours).cloned().to_set();

// A more robust definition of to_set might be self.cloned().collect()