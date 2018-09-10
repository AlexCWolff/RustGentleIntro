// maps.rs

/* Maps, also known as associative arrays or dicts, let you look up values associated with a key. This can also be done with an array of tuples. */

let entries = [("one","eins"),("two","zwei"),("three","drei")];

if let Some(val) = entries.iter().find(|t| t.0 == "two") {
    assert_eq!(val.1,"zwei");
}

/* This is fine for small maps and just requires equality to be defined for the keys, but the search takes linear time: it's proportional to the size of the map. A 'HashMap' does much better when there are a lot of key/value pairs to be searched. */

use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("one","eins");
map.insert("two","zwei");
map.insert("three","drei");

assert_eq! (map.contains_key("two"), true);
// 'get' returns a reference to the value, not the value itself. The value type is '&str', so we get a '&&str'. 
// In general it has to be a reference, because we can't just move a value out of its owning type.
assert_eq! (map.get("two"), Some(&"zwei"));

// 'get_mut' is like 'get' but returns a possible mutable reference. 
// Given a map from strings to integers, update the value for the key 'two'.
let mut map = HashMap::new();
map.insert("one",1);
map.insert("two",2);
map.insert("three",3);

println!("before {}", map.get("two").unwrap());

{
    let mut mref = map.get_mut("two").unwrap();
    *mref = 20;
}

println!("after {}", map.get("two").unwrap());
// before 2
// after 20

/* Note that getting that writable reference takes place in its own block. Otherwise, we would have a mutable borrow lasting until the end, and then Rust won't allow you to borrow from map again with 'map.get("two")'; it cannot allow any readable references while there's already a writable reference in scope. If it did, it could not guarantee that those readable references would remain valid. The solution is to make sure that mutable borrow doesn't last very long. It is not the most elegant API possible, but we can't throw away any possible errors. Python would bail out with an exception, and C++ would just create a default value. This is convenient but sneaky; easy to forget that the price of 'a_map["two"]' always returning an integer is that we can't tell the difference between zero and "not found", plus an extra entry is created. And no-one just calls 'unwrap', except in examples, however most Rust code you see consists of little standalone examples. It is much more likely for a match to take place: */

if let Some(v) = map.get("two") {
    let res = v + 1;
    assert_eq!(res, 3);
}
...
match map.get_mut("two") {
    Some(mref) => *mref = 20,
    None => panic!("_now_ we can panic!")
}
// We can iterate over the key/value pairs, but not in any particular order. There are also 'keys' and 'values' methods returning iterators over the keys and values respectively. 

for (k,v) in map.iter() {
    println!("key {} value {}", k,v);
}
// key one value eins
// key three value drei
// key two value zwei