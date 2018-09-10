/* Nom is a parser library for Rust which is well worth the initial time investment. If you have to parse a known data format, like CSV, JSON or configuration files, then it's best to use a specialized library. If the text is not regular, or some made-up format, then you need to scan that text without writing a lot of tedious string-processing code. The suggested go-to is often regex, but regexes can be frustratingly opaque. Nom provides a way to parse text which is just as powerful and can be built up by combining simpler parsers. Regexes have their limits; they should not be used for parsing HTML, but you could use Nom to parse HTML. If you ever want to write your own programming language Nom is a good place to start.

There are some excellent tutorials for learning Nom, but weI want to start at the hello-world level to build some initial familiarity. The basic things you need to know are that Nom is macros all the way down, and Nom prefers to work with byte slices instead of strings. Recent versions of Nom work fine with string slices, although you need to use the macros that end with '_s'. The first means that you have to be especially careful to get Nom expressions right, because the error messages are not going to be friendly. The second means that Nom can be used for any data format, not just text. People have used Nom to decode binary protocols and file headers, and it can also work with 'text' in encodings other than UTF-8. */

#[macro_use]
extern crate nom;

// this macro creates functions which take some input type ('&[u8]' by default) and return the second type in angle brackets
named!(get_greeting<&str,&str>,
    // matches a literal string in the stream of characters and its value is a string slice representing that literal
    // If you wanted to work with '&[u8]' then use the 'tag!' macro
    tag_s!("hi")
);

fn main() {
    // call the defined parser 'get_greeting' with a '&str' and get back an 'IResult'
    let res = get_greeting("hi there");
    println!("{:?}",res);
}
// Done(" there", "hi")
// " there" is the string slice left over after matching


// We want to ignore whitespace. By wrapping the 'tag!' in 'ws!' we can match "hi" anywhere among spaces, tabs or newlines
named!(get_greeting<&str,&str>,
    ws!(tag_s!("hi"))
);

fn main() {
    let res = get_greeting("hi there");
    println!("{:?}",res);
}
// Done("there", "hi")

// The result is "hi" as before, and the remaining string is "there". The spaces have been skipped.

// Let's match either "hi" or "bye"
// Note that you can use whitespace here to make the parser function easier to read
named!(get_greeting<&str>,
    // "alternate" macro takes parser expressions separated by '|' and matches any of them
    ws!(alt!(tag_s!("hi") | tag_s!("bye")))
);
println!("{:?}", get_greeting(" hi "));
println!("{:?}", get_greeting(" bye "));
println!("{:?}", get_greeting("  hola "));
// Done("", "hi")
// Done("", "bye")
// Error(Alt)

// The last match failed because there is no alternative that matches "hola".

// We need to understand this 'IResult' type to go further, but first let's compare this with the regex solution
let greetings = Regex::new(r"\s*(hi|bye)\s*").expect("bad regex");
let caps = greetings.captures(" hi ").expect("match failed");
println!("{:?}",caps);
// Captures({0: Some(" hi "), 1: Some("hi")})

/* Regular expressions are more compact. We needed to put '()' around the two possibilities separated by '|' so that we will capture the greeting and nothing else. The first result is the whole string, the second is the matched capture. '|' is the so-called 'alternation' operator in regexes, which is the motivation for the 'alt!' macro syntax. This is a very simple regex, and they get complicated very quickly. Being a text mini-language, you have to escape significant characters like '*' and '('. If we wanted to match "(hi)" or "(bye)" the regex becomes '\s*((hi|bye))\s*' but the Nom parser simply becomes 'alt!(tag_s!("(hi)") | tag_s!("(bye)"))'.

It's also a heavy-weight dependency. On my i5 laptop, Nom examples take about 0.55 seconds to compile, which is not much more than "Hello world". But the regex examples take about 0.90s. And the stripped release build executable of the Nom example is about 0.3Mb, which is about as small as statically linked Rust programs go, versus 0.8Mb for the regex example. */