// error2.rs

/* The 'simple-error' crate provides you with a basic error type based on a string, as we have defined it here, and a few convenient macros. Like any error, it works fine with 'Box<Error>' */

#[macro_use]
extern crate simple_error;

use std::error::Error;

type BoxResult<T> = Result<T,Box<Error>>;

fn run(s: &str) -> BoxResult<i32> {
    if s.len() == 0 {
        // 'bail!(s)' expands to return 'SimpleError::new(s).into();', return early with a conversion into the receiving type
        bail!("empty string");
    }
    Ok(s.trim().parse()?)
}

fn main() {
    println!("{:?}", run("23"));
    println!("{:?}", run("2x"));
    println!("{:?}", run(""));
}
// Ok(23)
// Err(ParseIntError { kind: InvalidDigit })
// Err(StringError("empty string"))

/* You need to use 'BoxResult' for mixing the 'SimpleError' type with other errors, since we can't implement 'From' for it, since both the trait and the type come from other crates. */