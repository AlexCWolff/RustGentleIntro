/* It is common to feel irritated with Rust strings in the beginning. There are different ways to create them, and they all feel verbose:

let s1 = "hello".to_string();
let s2 = String::from("dolly");
Isn't "hello" already a string? Well, in a way. String is an owned string, allocated on the heap; a string literal "hello" is of type &str ("string slice") and might be either baked into the executable ("static") or borrowed from a String. System languages need this distinction - consider a tiny microcontroller, which has a little bit of RAM and rather more ROM. Literal strings will get stored in ROM ("read-only") which is both cheaper and consumes much less power.

But (you may say) it's so simple in C++:

std::string s = "hello";
Which is shorter yes, but hides the implicit creation of a string object. Rust likes to be explicit about memory allocations, hence to_string. On the other hand, to borrow from a C++ string requires c_str, and C strings are stupid.

Fortunately, things are better in Rust - once you accept that both String and &str are necessary. The methods of String are mostly for changing the string, like push adding a char (under the hood it's very much like a Vec<u8>). But all the methods of &str are also available. By the same Deref mechanism, a String can be passed as &str to a function - which is why you rarely see &String in function definitions.

There are a number of ways to convert &str to String, corresponding to various traits. Rust needs these traits to work with types generically. As a rule of thumb, anything that implements Display also knows to_string, like 42.to_string().

Some operators may not behave according to intuition:

    let s1 = "hello".to_string();
    let s2 = s1.clone();
    assert!(s1 == s2);  // cool
    assert!(s1 == "hello"); // fine
    assert!(s1 == &s2); // WTF?
Remember, String and &String are different types, and == isn't defined for that combination. This might puzzle a C++ person who is used to references being almost interchangeable with values. Furthermore, &s2 doesn't magically become a &str, that's a deref coercion which only happens when assigning to a &str variable or argument. (The explicit s2.as_str() would work.)

However, this more genuinely deserves a WTF:

let s3 = s1 + s2;  // <--- no can do
You cannot concatenate two String values, but you can concatenate a String with a &str. You furthermore cannot concatenate a &str with a String. So mostly people don't use + and use the format! macro, which is convenient but not so efficient.

Some string operations are available but work differently. For instance, languages often have a split method for breaking up a string into an array of strings. This method for Rust strings returns an iterator, which you can then collect into a vector.

let parts: Vec<_> = s.split(',').collect();
This is a bit clumsy if you are in a hurry to get a vector. But you can do operations on the parts without allocating a vector! For instance, length of largest string in the split?

let max = s.split(',').map(|s| s.len()).max().unwrap();
(The unwrap is because an empty iterator has no maximum and we must cover this case.)

The collect method returns a Vec<&str>, where the parts are borrowed from the original string - we only need allocate space for the references. There is no method like this in C++, but until recently it would have to individually allocate each substring. (C++ 17 has std::string_view which behaves like a Rust string slice.) */