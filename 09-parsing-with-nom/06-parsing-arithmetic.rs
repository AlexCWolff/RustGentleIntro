/* With the necessary background established, we can do simple arithmetic expressions. This is a good example of something that really can't be done with regexes. The idea is to build up expressions from the bottom up. Expressions consist of terms, which are added or subtracted. Terms consist of factors, which are multiplied or divided. And (for now) factors are just floating-point numbers. */

named!(factor<f64>,
    ws!(float64)
);

named!(term<&str,f64>, do_parse!(
    init: factor >>
    res: fold_many0!(
        tuple!(
            alt!(tag_s!("*") | tag_s!("/")),
            factor
        ),
        init,
        |acc, v:(_,f64)| {
            if v.0 == "*" {acc * v.1} else {acc / v.1}
        }
    )
    >> (res)
));

named!(expr<&str,f64>, do_parse!(
    init: term >>
    res: fold_many0!(
        tuple!(
            alt!(tag_s!("+") | tag_s!("-")),
            term
        ),
        init,
        |acc, v:(_,f64)| {
            if v.0 == "+" {acc + v.1} else {acc - v.1}
        }
    )
    >> (res)
));

/* This expresses our definitions more precisely - an expression consists of at least one term, and then zero or many plus-or-minus terms. We don't collect them, but fold them using the appropriate operator. this is one of those cases where Rust can't quite work out the type of the expression, so we need a type hint. Doing it like this establishes the correct operator precedence.

We're going to need floating-point asserts here, and there's a crate for that. Add the line 'approx="0.1.1" to your Cargo.toml */

#[macro_use]
extern crate approx;
...
    assert_relative_eq!(fold_sum("1 2 3").to_result().unwrap(), 6.0);

/* Let's define a convenient little testing macro. 'stringify!' turns the expression into a string literal which we can feed into 'expr' and then compare the result with how Rust would evaluate it. */

macro_rules! expr_eq {
    ($e:expr) => (assert_relative_eq!(
        expr(stringify!($e).to_result().unwrap(),
        $e)
    )
}

expr_eq!(2.3);
expr_eq!(2.0 + 3.0 - 4.0);
expr_eq!(2.0*3.0 - 4.0);

/* In just a few lines we get an expression evaluator. But, we add an alternative to numbers in the factor parser; expressions contained inside parentheses. */

named!(factor<&str,f64>,
    alt!(
        ws!(float64) |
        ws!(delimited!( tag_s!("("), expr, tag_s!(")") ))
    )
);

expr_eq!(2.2*(1.1 + 4.5)/3.4);
expr_eq!((1.0 + 2.0)*(3.0 + 4.0*(5.0 + 6.0)));

/* Expressions are now defined recursively in terms of expressions. The  magic of 'delimited!' is that parentheses may be nested; Nom makes sure the brackets match up. We are now way past the capabilities of regular expressions, and the stripped executable at 0.5Mb is still half the size of a "hello world" regex program. */