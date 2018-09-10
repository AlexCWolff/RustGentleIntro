/* We've seen 'pairs!' and 'tuple!' which capture a fixed number of matches as Rust tuples. There is also 'many0' and 'many1'; they both capture indefinite numbers of matches as vectors. The difference is that the first may capture 'zero or many' and the second 'one or many'. 'many1!(ws!(float64))' would parse "1 2 3" into 'vec![1.0,2.0,3.0]', but will fail on the empty string.

'fold_many0' is a reducing operation. The match values are combined into a single value, using a binary operator. This is how Rust people did sums over iterators before 'sum' was added; this fold starts with an initial value (here zero) for the accumulator and keeps adding values to that accumulator using '+'. */

let res = [1,2,3].iter().fold(0,|acc,v| acc + v);
println!("{}",res);
// 6

// The Nom equivalent

named!(fold_sum<&str,f64>,
    fold_many1!(
        ws!(float64),
        0.0,
        |acc, v| acc + v
    )
);

println!("fold {}", fold_sum("1 2 3").to_result().unwrap());
//fold 6

// Up to now, we've had to capture every expression, or just grab all matching bytes with 'recognize!':

named!(pointf<(f64,&[u8],f64)>,
    tuple!(
        float64,
        tag_s!(","),
        float64
    )
);

println!("got {:?}", nom_res!(pointf,"20,52.2").unwrap());
//got (20, ",", 52.2)

/* For more complicated expressions, capturing the results of all the parsers leads to untidy types. 'do_parse!' lets you extract only the values you're interested in. The matches are separated with '>>'; the matches of interest are of the form 'name: parser'. */

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

named!(pointf<Point>,
    do_parse!(
        first: float64 >>
        tag_s!(",") >>
        second: float64
        >>
        (Point{x: first, y: second})
    )
);

println!("got {:?}", nom_res!(pointf,"20,52.2").unwrap());
// got Point { x: 20, y: 52.2 }

/* We're not interested in that tag's value (it can only be a comma) but we assign the two float values to temporary values which are used to build a struct. The code at the end can be any Rust expression. */