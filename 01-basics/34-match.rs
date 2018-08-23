// match.rs

/* The code in string3.rs where we extract the Russian greeting is not 
idiomatic. Instead, use 'match' 

match multilingual.find('п') {
    Some(idx) => {
        let hi = &multilingual[idx..];
        println!("Russian hi {}", hi);
    },
    None => println!("couldn't find the greeting, Товарищ")
};

'match' consists of several patterns with a matching value following the fat
arrow, separated by commas. It unwraps the value from the 'Option' and bound it
to 'idx'. You must specify all possibilities, including 'None'. This is more 
natural than the explicit 'is_some' check which needed an extra variable to 
store the Option. If you're not interested in failure here, then use 'if let'.

if let Some(idx) = multilingual.find('п') {
    println!("Russian hi {}", &multilingual[idx..]);
}

This is useful if you want to do a match and are only interested in one 
possible result. 'match' can operate like a switch statement, and can return a 
value.

let text = match n {
    0 => "zero",
    1 => "one",
    2 => "two",
    _ => "many",
};

The '_' is like C default. If you don't provide one then it will create a 
compiler error. Rust 'match' statements can also match on ranges. Note that 
these ranges have three dots and are inclusive ranges.

let text = match n {
    0...3 => "small",
    4...6 => "medium",
    _ => "large",
 };
*/