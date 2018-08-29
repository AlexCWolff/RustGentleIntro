// match2.rs

// Recall that the values of a tuple can be extracted with '()'
let t = (10,"hello".to_string());
...
let (n,s) = t;
// t has been moved. It is No More
// n is i32, s is String

/* This is a special case of destructuring; we have some data and wish to either pull it apart, like here, or just borrow its values. Either way we get the parts of a structure. The syntax is like that used in 'match'. */

// Here we are explicitly borrowing the values
let (ref n,ref s) = t;
// n and s are borrowed from t. It still lives!
// n is &i32, s is &String

// Destructuring works with structs as well:
struct Point {
    x: f32,
    y: f32
}

let p = Point{x:1.0,y:2.0};
...
let Point{x,y} = p;
// p still lives, since x and y can and will be copied
// both x and y are f32

// Revisiting 'match' with some new patterns. The first two patterns are exactly like 'let' destructuring
fn match_tuple(t: (i32,String)) {
    let text = match t {
        // only matches tuples with first element zero, but any string
        (0, s) => format!("zero {}", s),
        // adds an 'if' so that it only matches '(1,"hello")'
        (1, ref s) if s == "hello" => format!("hello one!"),
        // a variable will match anything, useful if the match applies to an expression and you don't want to bind a variable to that expression
        tt => format!("no match {:?}", tt),
        // '_ => format!("no match")' works like a variable but is ignored, a common way to finish off a 'match'
     };
    println!("{}", text);
}

/* You can't just match against '(1,"hello")', matching is exact and the compiler will complain.

= note: expected type `std::string::String`
= note:    found type `&'static str`

Why do we need 'ref s'? It's a somewhat obscure gotcha (E00008 error) where borrowing an if guard will cause a move when the if guard happens in a different context. This is a case of slight implementation leakage. */

// If the type was '&str' then we match it directly
match (42,"answer") {
    (42,"answer") => println!("yes"),
    _ => println!("no")
};

// What applies to 'match' also applies to 'if let'
let ot = Some((2,"hello".to_string());

// if we get a 'Some' we can match inside it and only extract the string from the tuple
if let Some((_,ref s)) = ot {
    assert_eq!(s, "hello");
}
 
/* It isn't necessary to have nested 'if let' statements here. We use '_' because we aren't interested in the first part of the tuple. We just borrowed the string, no 'destructive destructuring'. */

// An problem happens when using parse (or any function which needs to work out its return type from context)
if let Ok(n) = "42".parse() {
    // What's the type of 'n'? It's impossible to tell from context.
    ...
}

// You have to give a hint for the type of 'n' somehow
if let Ok(n) = "42".parse::<i32>() {
    ...
}

/* This somewhat non-elegant syntax is called the 'turbofish operator'. If you are in a function returning 'Result', then the question-mark operator provides a more elegant solution. */

let n: i32 = "42".parse()?;

/* However, the parse error needs to be convertible to the error type of the 'Result', which is a topic we'll take up later when discussing error handling. */