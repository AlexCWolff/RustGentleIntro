// error4.rs

/* The really cool thing that this crate provides is error chaining. As a library user, it's irritating when a method just 'throws' a generic I/O error. 'error_chain' does error chaining which helps solve this problem. When we try to open the file, we can lazily lean on the conversion to 'io::Error' using '?', or chain the error. */

// non-specific error
let f = File::open(&file)?;

// a specific chained error
let f = File::open(&file).chain_err(|| "unable to read the damn file")?;

// A new version of the program, with no imported 'foreign' errors, just the defaults:
#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{
    }

}
use errors::*;

fn run() -> Result<()> {
    use std::env::args;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    let file = args().skip(1).next()
        .ok_or(Error::from("filename needed"))?;

    ///////// chain explicitly! ///////////
    let f = File::open(&file).chain_err(|| "unable to read the damn file")?;

    let mut l = 0;
    for line in BufReader::new(f).lines() {
        let line = line.chain_err(|| "cannot read a line")?;
        println!("{}", line);
        l += 1;
        if l == 10 {
            break;
        }
    }

    Ok(())
}


fn main() {
    if let Err(e) = run() {
        println!("error {}", e);

        /////// look at the chain of errors... ///////
        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        std::process::exit(1);
    }
}
// $ cargo run foo
// error unable to read the damn file
// caused by: No such file or directory (os error 2)

/* The 'chain_err' method takes the original error and creates a new error which contains the original error; this can be continued indefinitely. The closure is expected to return any value which can be converted into an error. Rust macros can clearly save you a lot of typing. 'error-chain' even provides a shortcut that replaces the whole main program. */

quick_main!(run);
// run is where all the action takes place, anyway.