// args1.rs (aka args0.rs)

// Returns an iterator over the arguments as strings, including the program name
fn main() {
    for arg in std::env::args() {
        println!("'{}'", arg);
    }
}

/* You can also use 'collect' to make that vector, use the iterator 'skip' 
method to move past the program name.

let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 0 { // we have args!
        ...
    }
*/