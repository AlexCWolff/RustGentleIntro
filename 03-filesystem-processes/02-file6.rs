// file6.rs

// The 'write!' macro works with anything that implements 'Write', so here's a another way of saying 'print!'
let mut stdout = io::stdout();
...
write!(stdout,"answer is {}\n", 42).expect("write failed");
    
/* If an error is possible, you must handle it. It may not be very likely but it can happen. If you are doing file i/o you should be in a context where '?' works. There is a difference: 'print!' locks stdout for each write. This is usually what you want for output because without that locking multithreaded programs can mix up that output in interesting ways. If you are pumping out a lot of text then 'write!' is going to be faster. For arbitrary files we need 'write!' because the file is closed when 'out' is dropped at the end of 'write_out'. */

use std::fs::File;
use std::io;
use std::io::prelude::*;

fn write_out(f: &str) -> io::Result<()> {
    let mut out = File::create(f)?;
    write!(out,"answer is {}\n", 42)?;
    Ok(())
}

fn main() {
  write_out("test.txt").expect("write failed");
}

/* Rust files are unbuffered by default, so each write request goes straight to the OS, and this is going to be slow. This default is different from other programming languages so Rust can be be slower than scripting languages without proper planning. Just as with 'Read' and 'io::BufReader', there is 'io::BufWriter' for buffering any 'Write'. */