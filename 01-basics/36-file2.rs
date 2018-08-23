// file2.rs
use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

/* This version of the file reading function does not crash, it returns a 
'Result' and the caller who must decide how to handle the error. */

fn read_to_string(filename: &str) -> Result<String,io::Error> {
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
}

fn main() {
    let file = env::args().nth(1).expect("please supply a filename");

    let text = read_to_string(&file).expect("bad file man!");

    println!("file had {} bytes", text.len());
}

/* The first match safely extracts the value from 'Ok', which becomes the value of the match. If it's 'Err' it returns the error, rewrapped as an 'Err'. The second match returns the string, wrapped up as an 'Ok', otherwise an error. The actual value in the 'Ok' is unimportant, so we ignore it with '_'.

This is not ugly, most of a function is error handling. Go tends to have this problem, with lots of explicit early returns, or just ignoring errors, which you should never do. Fortunately, there is a shortcut. The 'std::io' module defines a type alias 'io::Result<T>' which is the same as 'Result<T,io::Error>' and easier to type.

fn read_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

The '?' operator does almost what the match on 'File::open' does, if the result was an error, then immediately return that error, otherwise it returns 'Ok'. We still need to wrap up the string as a result. You will still see the macro 'try!' used in older code

fn read_to_string(filename: &str) -> io::Result<String> {
    let mut file = try!(File::open(&filename));
    let mut text = String::new();
    try!(file.read_to_string(&mut text));
    Ok(text)
} 

It's possible to write safe Rust that isn't ugly, without needing exceptions.
*/