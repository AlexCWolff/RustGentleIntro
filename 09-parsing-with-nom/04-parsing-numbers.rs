// Nom provides a function 'digit' which matches a series of numerical digits. 
// We use 'map!' to convert the string into an integer, and return the full 'Result' type.
use nom::digit;
use std::str::FromStr;
use std::num::ParseIntError;

named!(int8 <&str, Result<i8,ParseIntError>>,
    map!(digit, FromStr::from_str)
);

named!(int32 <&str, Result<i32,ParseIntError>>,
    map!(digit, FromStr::from_str)
);

println!("{:?}", int8("120"));
println!("{:?}", int8("1200"));
println!("{:?}", int8("x120"));
println!("{:?}", int32("1202"));

// Done("", Ok(120))
// Done("", Err(ParseIntError { kind: Overflow }))
// Error(Digit)
// Done("", Ok(1202))

/* What we get is a parser 'IResult' containing a conversion 'Result', there is more than one way to fail here. Note that the body of our converting function has exactly the same code; the actual conversion depends on the return type of the function. Integers may have a sign. We can capture integers as a pair, where the first value may be a sign, and the second value would be any digits following. */

named!(signed_digits<&str, (Option<&str>,&str)>,
    pair!(
        opt!(alt!(tag_s!("+") | tag_s!("-"))),  // maybe sign?
        digit
    )
);

println!("signed {:?}", signed_digits("4"));
println!("signed {:?}", signed_digits("+12"));
// signed Done("", (None, "4"))
// signed Done("", (Some("+"), "12"))

// When we aren't interested in the intermediate results, but just want all the matching input, then 'recognize!' is what you need.

named!(maybe_signed_digits<&str,&str>,
    recognize!(signed_digits)
);

println!("signed {:?}", maybe_signed_digits("+12"));
// signed Done("", "+12")

/* With this technique we can recognize floating-point numbers. Again we map to string slice from the byte slice over all these matches. 'tuple!' is the generalization of 'pair!', although we aren't interested in the generated tuple here. 'complete!' is needed to resolve the same problem we had with incomplete greetings; "12" is a valid number without the optional floating-point part. */

named!(floating_point<&str,&str>,
    recognize!(
        tuple!(
            maybe_signed_digits,
            opt!(complete!(pair!(
                tag_s!("."),
                digit
            ))),
            opt!(complete!(pair!(
                alt!(tag_s!("e") | tag_s!("E")),
                maybe_signed_digits
            )))
        )
    )
);
// By defining a helper macro, we get some passing tests
// The test passes if 'floating_point' matches all of the string that it is given.

macro_rules! nom_eq {
    ($p:expr,$e:expr) => (
        assert_eq!($p($e).to_result().unwrap(), $e)
    )
}

nom_eq!(floating_point, "+2343");
nom_eq!(floating_point, "-2343");
nom_eq!(floating_point, "2343");
nom_eq!(floating_point, "2343.23");
nom_eq!(floating_point, "2e20");
nom_eq!(floating_point, "2.0e-6");

// Although sometimes macros feel a little dirty, making your tests pretty is nice.

// Then we can parse and convert floating point numbers. 
// Here we'll take a risk and throw away the error
named!(float64<f64>,
    map_res!(floating_point, FromStr::from_str)
);
// Note how it's possible to build up complicated parsers step by step, testing each part in isolation first. That's a strong advantage of parser combinators over regexes. 