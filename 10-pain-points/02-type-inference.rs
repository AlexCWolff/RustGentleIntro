/* The distinction between 'static' and 'dynamic' isn't everything. Like with most things, there are more dimensions in play. C is statically-typed, every variable has a type at compile-time, but weakly-typed, e.g. 'void*' can point to anything; Python is dynamically-typed, the type is in the value, not the variable, but strongly-typed. Java is static/sorta strong, with reflection as convenient/dangerous escape valve, and Rust is static/strong, with no runtime reflection.

Java is famous for needing all thoses types typed out in numbing detail, Rust likes to infer types. This is generally a good idea, but it does mean that you sometimes need to work out what the actual types are. You will see let 'n = 100' and wonder what kind of integer is this? By default, it would be 'i32', a four-byte signed integer. Everyone agrees by now that C's unspecified integer types, like 'int' and 'long', are a bad idea; better to be explicit. You can always spell out the type, as in 'let n: u32 = 100' or let the literal force the type, as in 'let n = 100u32', but type inference goes much further than that. If you declare 'let n = 100' then all 'rustc' knows that 'n' must be some integer type. If you then passed 'n' to a function expecting a 'u64' then that must be the type of 'n'.

After that, you try to pass 'n' to a function expecting 'u32'. 'rustc' will not let you do this because 'n' has been tied down to 'u64' and it will not  convert that integer for you. This is strong typing in action; there are none of those little conversions and promotions which make your life smoother until an integer overflow bites. You would have to explicitly pass 'n' as 'n' as 'u32', a Rust typecast. Fortunately, 'rustc' is good at telling you this in an actionable way; you can follow the compiler's advice about fixing the problem. */

// Rust code can often be free of explicit types
let mut v = Vec::new();
// v is deduced to have type Vec<i32>
v.push(10);
v.push(20);
v.push("hello") // you just can't do this

/* Not being able to put strings into a vector of integers is a feature, not a bug. The flexibility of dynamic typing is also a curse. If you do need to put integers and strings into the same vector, then Rust 'enum' types are the way to do it safely. Sometimes you need to at least give a type hint. 'collect' is a fantastic iterator method, but it needs a hint. Say we have a iterator returning 'char', 'collect' can swing two ways. */

// a vector of char ['h','e','l','l','o']
let v: Vec<_> = "hello".chars().collect();
// a string "dolly"
let m: String = "dolly".chars().filter(|&c| c != 'l').collect();

// When feeling uncertain about the type of a variable, there's a trick to force 'rustc' to reveal the actual type name in an error message:

let x: () = var;

// 'rustc' may pick an over-specific type. Here we want to put different references into a vector as '&Debug' but need to declare the type explicitly.
use std::fmt::Debug;

let answer = 42;
let message = "hello";
let float = 2.7212;

let display: Vec<&Debug> = vec![&message, &answer, &float];

for d in display {
    println!("got {:?}", d); 
}