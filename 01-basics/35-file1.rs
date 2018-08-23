// file1.rs

/* Remember that 'expect' is like 'unwrap' but gives a custom error message. 
We are going to throw away a few errors. */

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let first = env::args().nth(1).expect("please supply a filename");

    let mut file = File::open(&first).expect("can't open the file");

    let mut text = String::new();
    file.read_to_string(&mut text).expect("can't read the file");

    println!("file had {} bytes", text.len());
}

/* 
src$ file1 file1.rs
file had 366 bytes
src$ ./file1 frodo.txt
thread 'main' panicked at 'can't open the file: Error { repr: Os { code: 2, message: "No such file or directory" } }', ../src/libcore/result.rs:837
note: Run with `RUST_BACKTRACE=1` for a backtrace.
src$ file1 file1
thread 'main' panicked at 'can't read the file: Error { repr: Custom(Custom { kind: InvalidData, error: StringError("stream did not contain valid UTF-8") }) }', ../src/libcore/result.rs:837
note: Run with `RUST_BACKTRACE=1` for a backtrace.

'open' can fail because the file doesn't exist or we aren't allowed to read it, 'read_to_string' can fail because the file doesn't contain valid UTF-8. You can use 'read_to_end' and put the contents into a vector of bytes instead. For small files that reading them all at once useful and easy. The file is closed automatically when the function goes out of scope.

You do not want to put this code into a function, knowing that it could so easily crash the whole program. Now we have to talk about exactly what 'File::open' returns; 'Option' is a value that may contain something or nothing, 'Result' is a value that may contain something or an error. They both understand 'unwrap' and 'expect' but they are different. Result is defined by two type parameters, for the 'Ok' value and the 'Err' value.
*/

fn good_or_bad(good: bool) -> Result<i32,String> {
    if good {
        Ok(42)
    } else {
        Err("bad".to_string())
    }
}

fn main() {
    println!("{:?}",good_or_bad(true));
    //Ok(42)
    println!("{:?}",good_or_bad(false));
    //Err("bad")

    match good_or_bad(true) {
        Ok(n) => println!("Cool, I got {}",n),
        Err(e) => println!("Huh, I just got {}",e)
    }
    // 42

}
/* The 'error' type is arbitrary, it's just a convenient way to either return one value or another. Many people use strings until they are comfortable with Rust error types. */