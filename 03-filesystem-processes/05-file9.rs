// file9.rs

// A program which searches for a configuration file that may appear in any subdirectory of the current directory. 
use std::env;

fn main() {
    let mut path = env::current_dir().expect("can't access current dir");
    loop {
        path.push("config.txt");
        if path.is_file() {
            println!("gotcha {}", path.display());
            break;
        } else {
            path.pop();
        }
        if ! path.pop() {
            break;
        }
    }
}
// gotcha /home/steve/rust/config.txt

// This is pretty much how git works when it wants to know what the current repo is.