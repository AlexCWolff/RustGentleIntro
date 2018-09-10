/* 'IResult' is different from the standard 'Result' type in that there are three possibilities:

* 'Done' - success - you get both the result and the remaining bytes
* 'Error' - failed to parse - you get an error
* 'Incomplete' - more data needed

We can write a generic 'dump' function that handles any return value that can be debug-printed. This also demonstrates the 'to_result' method which returns a regular 'Result'; this is probably the method you will use for most cases since it returns either the returned value or an error. */

#[macro_use]
extern crate nom;
use nom::IResult;
use std::str::from_utf8;
use std::fmt::Debug;

fn dump<T: Debug>(res: IResult<&str,T>) {
    match res {
      IResult::Done(rest, value) => {println!("Done {:?} {:?}",rest,value)},
      IResult::Error(err) => {println!("Err {:?}",err)},
      IResult::Incomplete(needed) => {println!("Needed {:?}",needed)}
    }
}


fn main() {
    named!(get_greeting<&str,&str>,
        ws!(
            alt!( tag_s!("hi") | tag_s!("bye"))
        )
    );

    dump(get_greeting(" hi "));
    dump(get_greeting(" bye hi"));
    dump(get_greeting("  hola "));

    println!("result {:?}", get_greeting(" bye  ").to_result());
}
// Done Ok("") "hi"
// Done Ok("hi") "bye"
// Err Alt
// result Ok("bye")

// Parsers returning any unparsed text, and being able to indicate that they don't have enough input characters to decide, is very useful for stream parsing, but normally you want to use 'to_result'