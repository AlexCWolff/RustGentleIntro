// move1.rs

fn main() {
    let s1 = "hello dolly".to_string();
    let s2 = s1;
    println!("s1 {}", s1);
}

/* 
error[E0382]: use of moved value: `s1`
 --> move1.rs:5:22
  |
4 |     let s2 = s1;
  |         -- value moved here
5 |     println!("s1 {}", s1);
  |                      ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`,
  which does not implement the `Copy` trait

In a language where variables are always references like Java or Python, 's2' 
becomes another reference to the string object referenced by 's1'. In C++, 's1' 
is a value and it is copied to 's2'. But Rust moves the value. It doesn't see 
strings as copyable ("does not implement the Copy trait"). We would not see this
with primitive types since they are just values, they are allowed to be copied 
because they are cheap to copy. 'String' has allocated memory and copying will 
involve allocating more memory and copying the characters.

Consider a 'String' containing the whole text of 'Moby-Dick'. It's not a big 
struct, just has the address in memory of the text, its size, and how big the 
allocated block is. Copying this is going to be expensive, that memory is 
allocated on the heap and the copy will need its own allocated block.

    String
    | addr | ---------> Call me Ishmael.....
    | size |                    |
    | cap  |                    |
                                |
    &str                        |
    | addr | -------------------|
    | size |

    f64
    | 8 bytes |

The second value is a string slice '&str' which refers to the same memory as 
the string, with a size, but only the guys name. This is cheap to copy. The 
third value is an 'f64', just 8 bytes. It does not refer to any other memory 
so it's also as cheap to copy as to move. 'Copy' values are only defined by 
their representation in memory, and when Rust copies it just copies those bytes 
elsewhere. A non-'Copy' value is also just moved. There is no cleverness in 
copying and moving, unlike in C++. */