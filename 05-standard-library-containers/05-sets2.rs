// sets2.rs

/* It's often useful to have an interactive session with a program. Each line is read in and split into words; the command is looked up on the first word, and the rest of the words are passed as an argument to that command. A natural implementation is a map from command names to closures. But how do we store closures, given that they will all have different sizes? Boxing them will copy them onto the heap:. */


    let mut v = Vec::new();
    v.push(Box::new(|x| x * x));
    v.push(Box::new(|x| x / 2.0));

    for f in v.iter() {
        let res = f(1.0);
        println!("res {}", res);
    }

/* We get a very definite error on the second push:

  = note: expected type `[closure@closure4.rs:4:21: 4:28]`
  = note:    found type `[closure@closure4.rs:5:21: 5:28]`
note: no two closures, even if identical, have the same type

'rustc' has deduced a type which is too specific, so it's necessary to force that vector to have the boxed trait type before it work. */

let mut v: Vec<Box<Fn(f64)->f64>> = Vec::new();

/* We can now use the same trick and keep these boxed closures in a 'HashMap'. We still have to watch out for lifetimes since closures can borrow from their environment. It's tempting to make them 'FnMut' so they can modify any captured variables, but we will have more than one command, each with its own closure, and you cannot then mutably borrow the same variables. The closures are passed a mutable reference as an argument, plus a slice of string slices ('&[&str]') representing the command arguments. They will return some 'Result', and ee'll use 'String' errors at first. 'D' is the data type, which can be anything with a size. */ 

type CliResult = Result<String,String>;

struct Cli<'a,D> {
    data: D,
    callbacks: HashMap<String, Box<Fn(&mut D,&[&str])->CliResult + 'a>>
}

impl <'a,D: Sized> Cli<'a,D> {
    fn new(data: D) -> Cli<'a,D> {
        Cli{data: data, callbacks: HashMap::new()}
    }
    
    // 'cmd' is passed a name and any closure that matches our signature, which is boxed and entered into the map
    fn cmd<F>(&mut self, name: &str, callback: F)
    // 'Fn' means that our closures borrow their environment but can't modify it
    where F: Fn(&mut D, &[&str])->CliResult + 'a {
        self.callbacks.insert(name.to_string(),Box::new(callback));
    }

/* 'Fn' is a generic method where the declaration is scarier than the implementation. Forgetting the explicit lifetime is a common error here, Rust won't let us forget that these closures have a lifetime limited to their environment. */

// Reading and running commands
fn process(&mut self,line: &str) -> CliResult {
    // split the line into words as a vector
    let parts: Vec<_> = line.split_whitespace().collect();
    // An empty line is ignored and not considered an error
    if parts.len() == 0 {
        return Ok("".to_string());
    }
    // Look up the first word in the map, call the closure with our stored mutable data and the rest of the words
    match self.callbacks.get(parts[0]) {
        Some(callback) => callback(&mut self.data,&parts[1..]),
        None => Err("no such command".to_string())
    }
}

fn go(&mut self) {
    let mut buff = String::new();
    while io::stdin().read_line(&mut buff).expect("error") > 0 {
        {
            let line = buff.trim_left();
            let res = self.process(line);
            println!("{:?}", res);

        }
        buff.clear();
    }
}

/* Next, define some helper functions to make it easier for our closures to return correct and incorrect results. They are generic functions that work for any type that can be converted to a 'String'. */

fn ok<T: ToString>(s: T) -> CliResult {
    Ok(s.to_string())
}

fn err<T: ToString>(s: T) -> CliResult {
    Err(s.to_string())
}

// The main program.
use std::error::Error;

fn main() {
    println!("Welcome to the Interactive Prompt! ");

    struct Data {
        answer: i32
    }

    let mut cli = Cli::new(Data{answer: 42});

    cli.cmd("go",|data,args| {
        if args.len() == 0 { return err("need 1 argument"); }
        data.answer = match args[0].parse::<i32>() {
            Ok(n) => n,
            Err(e) => return err(e.description())
        };
        println!("got {:?}", args);
        // 'ok(answer)' works because integers know how to convert themselves to strings
        ok(data.answer)
    });

    cli.cmd("show",|data,_| {
        ok(data.answer)
    });

    cli.go();
}

/* The error handling is a bit clunky here, later we'll see how to use the question mark operator in cases like this. The particular error 'std::num::ParseIntError' implements the trait 'std::error::Error', which we must bring into scope to use the 'description' method; Rust doesn't let traits operate unless they're visible. Output:

Welcome to the Interactive Prompt!
go 32
got ["32"]
Ok("32")
show
Ok("32")
goop one two three
Err("no such command")
go 42 one two three
got ["42", "one", "two", "three"]
Ok("42")
go boo!
Err("invalid digit found in string")

Some obvious improvements to try: if we give 'cmd' three arguments with the second being a help line then we can store these help lines and automatically implement a 'help' command, also having some command editing and history is very convenient so use the 'rustyline' crate. */