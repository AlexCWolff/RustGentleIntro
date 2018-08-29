// file5.rs

/* Reading a whole file into a string isn't always a good idea, instead you should read a file line-by-line. 'fs::File' implements 'io::Read', which is the trait for anything readable. This trait defines a read method which will fill a slice of 'u8' with bytes. This is the only required method of the trait, and you get some methods for free, like with 'Iterator'. You can use 'read_to_end' to fill a vector of bytes from the readable, and 'read_to_string' to fill a UTF-8 encoded string.

This is a "raw" read with no buffering, for buffered reading there is the 'io::BufRead' trait which gives us 'read_line' and a 'lines' iterator. 'io::BufReader' will provide an implementation of 'io::BufRead' for any readable. 'fs::File' also implements 'io::Write'. The easiest way to make sure all these traits are visible is use 'std::io::prelude::*'. */

use std::fs::File;
use std::io;
use std::io::prelude::*;

fn read_all_lines(filename: &str) -> io::Result<()> {
    let file = File::open(&filename)?;

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }
    Ok(())
}

/* The 'let line = line?' is needed since the line returned by the iterator is  an 'io::Result<String>' which we unwrap with '?', because things can go wrong during this iteration: I/O errors, receiving bytes that aren't UTF-8, etc.

Since 'lines' is an iterator it is straightforward to read a file into a vector of strings using 'collect' or print out the line with line numbers using 'enumerate'. It isn't the most efficient way to read all the lines, however, because a new string is allocated for each line. It is more efficient to use 'read_line'. Note that the returned line includes the linefeed, which can be removed using 'trim_right'. */

let mut reader = io::BufReader::new(file);
let mut buf = String::new();
while reader.read_line(&mut buf)? > 0 {
    {
        let line = buf.trim_right();
        println!("{}", line);
    }
    buf.clear();
}

/* This results in far less allocations because clearing that string does not free its allocated memory; once the string has enough capacity, no more allocations will take place. This is a case using a block to control a borrow. 'line' is borrowed from 'buf', and this borrow must finish before we modify 'buf'. Rust is trying to stop us doing something stupid, i.e access 'line' after we've cleared the buffer. (

The borrow checker can be restrictive sometimes. Rust is due to get 'non-lexical lifetimes', where it will analyze the code and see that line isn't used after 'buf.clear()'. */

// You can't do a proper iterator that returns references to a buffer, but this is similar
use std::fs::File;
use std::io;
use std::io::prelude::*;

// Type parameter 'R' is 'any type that implements Read'
struct Lines<R> {
    reader: io::BufReader<R>,
    buf: String
}

// The reader and the buffer which we are going to borrow from
impl <R: Read> Lines<R> {
    fn new(r: R) -> Lines<R> {
        Lines{reader: io::BufReader::new(r), buf: String::new()}
    }
    ...
}

// Returns an 'Option', when it returns None it finishes just like an iterator
// The return type is a 'Result' because 'read_line' might fail, and we should never throw errors away
fn next<'a>(&'a mut self) -> Option<io::Result<&'a str>>{
    self.buf.clear();
    match self.reader.read_line(&mut self.buf) {
        Ok(nbytes) => if nbytes == 0 {
            None // no more lines!
        } else {
            // The buffer contains the line with a linefeed (`\n') appended.
            // Trim this away, and package up the string slice.
            let line = self.buf.trim_right();
            Some(Ok(line))
        },
        // If it fails, we wrap up its error in a Some<Result>
        Err(e) => Some(Err(e))
    }
}

/* We need an explicit lifetime because Rust will never allow us to hand out borrowed string slices without knowing their lifetime. Here we say that the lifetime of this borrowed string is within the lifetime of 'self'. This signature, with the lifetime, is incompatible with the interface of 'Iterator'. It's easy to see problems if it were compatible; consider 'collect' trying to make a vector of these string slices. This can't work since they're all borrowed from the same mutable string. If you had read all the file into a string, then the string's 'lines' iterator can return string slices because they are all borrowed from distinct parts of the original string. */

// Much cleaner, and the file buffering is invisible to the user
fn read_all_lines(filename: &str) -> io::Result<()> {
    let file = File::open(&filename)?;

    let mut lines = Lines::new(file);
    while let Some(line) = lines.next() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}

// You can also write the loop like this since the explicit match can pull out the string slice
while let Some(Ok(line)) = lines.next() {
    println!("{}", line)?;
}

/* But you are throwing away a possible error here; this loop will silently stop whenever an error occurs. It will stop at the first place where Rust can't convert a line to UTF-8. */