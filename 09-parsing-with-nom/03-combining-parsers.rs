// Continuing the greeting example and imagine that a greeting consists of "hi" or "bye", plus a name.
named!(full_greeting<&str,(&str,&str)>,
    // 'pair!' macro will collect the result of matching two parsers as a tuple
    pair!(
        get_greeting,
        // 'nom::alpha' matches a series of alphabetical characters
        nom::alpha
    )
);

println!("result {:?}", full_greeting(" hi Bob  ").to_result());
// result Ok(("hi", "Bob"))

// Let's make the name optional. The second value of the tuple becomes an 'Option'.
named!(full_greeting<&str, (&str,Option<&str>)>,
    pair!(
        get_greeting,
        opt!(nom::alpha)
    )
);

println!("result {:?}", full_greeting(" hi Bob  ").to_result());
println!("result {:?}", full_greeting(" bye ?").to_result());
// result Ok(("hi", Some("Bob")))
// result Ok(("bye", None))


/* It was straightforward to combine an existing parser for greetings with a parser that picks up names, and then it was easy to make that name optional. This is the power of Nom, and why it's called a "parser combinator library". You can build up your complicated parsers from simpler parsers, which you can test individually. At this point, the equivalent regex is starting to look like a Perl program; regexes do not combine well. We're not done yet though. 'full_greeting(" bye ")' will fail with an 'Incomplete' error. Nom knows that "bye" may be followed by a name and wants us to give it more data. This is how a streaming parser needs to work, so you can feed it a file chunk by chunk, but here we need to tell Nom that the input is complete. */

named!(full_greeting<&str,(&str,Option<&str>)>,
    pair!(
        get_greeting,
        opt!(complete!(nom::alpha))
    )
);

println!("result {:?}", full_greeting(" bye ").to_result());
// result Ok(("bye", None)) */