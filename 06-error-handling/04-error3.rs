// error3.rs

/* For non-trivial applications have a look at the 'error_chain' crate. A little macro magic can go a long way in Rust.

Create a binary crate with 'cargo new --bin test-error-chain' and change to this directory. Edit 'Cargo.toml' and add 'error-chain="0.8.1"' to the end.

'error-chain' creates all the definitions we needed for manually implementing an error type; creating a struct, and implementing the necessary traits: 'Display', 'Debug' and 'Error'. It also by implements 'From' so strings can be converted into errors.

All the main program does is call 'run', print out any errors, and end the program with a non-zero exit code. The macro 'error_chain' generates all the definitions needed, within an 'error' module; in a larger program you would put this in its own file. We need to bring everything in 'error' back into global scope because our code will need to see the generated traits. By default, there will be an 'Error' struct and a 'Result' defined with that error. We ask for 'From' to be implemented so that 'std::io::Error' will convert into our error type using 'foreign_links'*/

#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
        }
    }
}
use errors::*;

fn run() -> Result<()> {
    use std::fs::File;

    File::open("file")?;

    Ok(())
}


fn main() {
    if let Err(e) = run() {
        println!("error: {}", e);

        std::process::exit(1);
    }
}
// error: No such file or directory (os error 2)

/* The 'foreign_links' has made our life easier, since '?' now knows how to convert 'std::io::Error' into our 'error::Error'. Under the hood, the macro is creating a 'From<std::io::Error>' conversion, exactly as spelt out earlier. 

All the action happens in 'run'; let's make it print out the first 10 lines of a file given as the first program argument. There may or may not be such an argument, which isn't necessarily an error. Here we want to convert an Option<String> into a Result<String>. There are two Option methods for doing this conversion, and I've picked the simplest one. Our 'Error' type implements 'From' for '&str', so it's straightforward to make an error with a simple text message. */

fn run() -> Result<()> {
    use std::env::args;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    let file = args().skip(1).next()
        .ok_or(Error::from("provide a file"))?;

    let f = File::open(&file)?;
    let mut l = 0;
    for line in BufReader::new(f).lines() {
        let line = line?;
        println!("{}", line);
        l += 1;
        if l == 10 {
            break;
        }
    }

    Ok(())
}

// There is another useful little macro 'bail!' for 'throwing' errors. An alternative to the 'ok_or' method here could be:
let file = match args().skip(1).next() {
    Some(s) => s,
    None => bail!("provide a file")
};

/* Like '?' it does an early return. The returned error contains an enum 'ErrorKind', which allows us to distinguish between various kinds of errors. There's always a variant 'Msg' (when you say 'Error::from(str)') and the 'foreign_links' has declared 'Io' which wraps I/O errors: */

fn main() {
    if let Err(e) = run() {
        match e.kind() {
            &ErrorKind::Msg(ref s) => println!("msg {}",s),
            &ErrorKind::Io(ref s) => println!("io {}",s),
        }
        std::process::exit(1);
    }
}
// $ cargo run
// msg provide a file
// $ cargo run foo
// io No such file or directory (os error 2)

// To add new kinds of errors, add an errors section to the 'error_chain!' macro:
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
        }

        errors {
            NoArgument(t: String) {
                // This defines how 'Display' works for this new kind of error.
                display("no argument provided: '{}'", t)
            }
        }

    }

// Now we can handle 'no argument' errors more specifically, feeding 'ErrorKind::NoArgument' a 'String' value:
let file = args().skip(1).next()
    .ok_or(ErrorKind::NoArgument("filename needed".to_string()))?;

// There's now an extra 'ErrorKind' variant that you must match
fn main() {
    if let Err(e) = run() {
        println!("error {}",e);
        match e.kind() {
            &ErrorKind::Msg(ref s) => println!("msg {}", s),
            &ErrorKind::Io(ref s) => println!("io {}", s),
            &ErrorKind::NoArgument(ref s) => println!("no argument {:?}", s),
        }
        std::process::exit(1);
    }
}
// cargo run
// error no argument provided: 'filename needed'
// no argument "filename needed"

/* Generally, it's useful to make errors as specific as possible, particularly in a library function. This match-on-kind technique is very similar to traditional exception handling, where you match on exception types in a catch or except block. 'error-chain' creates a type 'Error' for you, and defines 'Result<T>' to be 'std::result::Result<T,Error>'. 'Error' contains an enum 'ErrorKind' and by default there is one variant 'Msg' for errors created from strings. You define external errors with 'foreign_links' which does two things: creates a new 'ErrorKind' variant, and defines 'From' on these external errors so they can be converted to our error. New error variants can be easily added, and a lot of boilerplate code is eliminated. */