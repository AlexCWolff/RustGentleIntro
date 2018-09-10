// box-errors.rs

/* Error handling in Rust can be clumsy if you can't use the question-mark operator. We need to return a 'Result' which can accept any error, and all errors implement the trait 'std::error::Error', so any error can convert into a 'Box<Error>'. */

// Handle both i/o errors and errors from converting strings into numbers
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn run(file: &str) -> Result<i32,Box<Error>> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().parse()?)
}

/* So that's two question-marks for the i/o errors (can't open file, or can't read as string) and one question-mark for the conversion error. Finally, we wrap the result in 'Ok'. Rust can work out from the return type that 'parse' should convert to 'i32'. */

// A shortcut for this 'Result' type
type BoxResult<T> = Result<T,Box<Error>>;