// error1.rs

/* Programs will have application-specific error conditions, so we need to create our own error type. Your error can do pretty much what it likes, but the basic requirements are: may implement 'Debug', must implement 'Display', and must implement 'Error'. */

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

// a test function that returns our error result
fn raises_my_error(yes: bool) -> Result<(),MyError> {
    if yes {
        Err(MyError::new("borked"))
    } else {
        Ok(())
    }
}

/* Typing 'Result<T,MyError>' gets tedious and many Rust modules define their own 'Result', e.g. 'io::Result<T>' is short for 'Result<T,io::Error>'. We need to handle the specific error when a string can't be parsed as a floating-point number. The way that '?' works is to look for a conversion from the error of the expression to the error that must be returned. And this conversion is expressed by the 'From' trait. 'Box<Error>' works as it does because it implements 'From' for all types implementing 'Error'. You can continue to use the convenient alias 'BoxResult' and catch everything as before; there will be a conversion from our error into 'Box<Error>'. This is a good option for smaller applications, but other errors can be explicitly made to cooperate with our error type. */

// 'ParseFloatError' implements 'Error' so 'description()' is defined.
use std::num::ParseFloatError;

impl From<ParseFloatError> for MyError {
    fn from(err: ParseFloatError) -> Self {
        MyError::new(err.description())
    }
}

// test
fn parse_f64(s: &str, yes: bool) -> Result<f64,MyError> {
    // This '?' is fine, a type always converts to itself with 'From'
    raises_my_error(yes)?;
    // This '?' will convert the 'ParseFloatError' to 'MyError'
    let x: f64 = s.parse()?;
    Ok(x)
}

// results
fn main() {
    println!(" {:?}", parse_f64("42",false));
    println!(" {:?}", parse_f64("42",true));
    println!(" {:?}", parse_f64("?42",false));
}
//  Ok(42)
//  Err(MyError { details: "borked" })
//  Err(MyError { details: "invalid float literal" })

/* The tedious bit is having to write 'From' conversions for all the other error types that need to play nice with 'MyError', or you can lean on 'Box<Error>'. The multitude of ways to do the same thing can be confusing. Error-handling for a 200 line program can afford to be simpler than for a large application. And if you ever want to package your code as a Cargo crate, then error handling becomes crucial. Currently '?' only works for 'Result', not 'Option', and this is deliberate. 'Option' has a 'ok_or_else' which converts itself into a 'Result'. */

// Say we had a 'HashMap' and must fail if a key isn't defined
let val = map.get("my_key").ok_or_else(|| MyError::new("my_key not defined"))?;
// The error returned is completely clear. This form uses a closure, so the error value is only created if the lookup fails.