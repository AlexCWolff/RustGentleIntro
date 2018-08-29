// file7.rs

use std::env;
use std::path::PathBuf;

// Print out the Cargo directory on a machine
// Will fail if no homedir exists, but that's pretty much impossible
fn main() {
    // 'env::home_dir' is cross-platform
    let home = env::home_dir().expect("no home!");
    // create a 'PathBuf'
    let mut path = PathBuf::new();
    // Use its push method to build up the full file path from its components
    path.push(home);
    path.push(".cargo");

    if path.is_dir() {
        println!("{}", path.display());
    }
}

/* A 'PathBuf' is like 'String'; it owns a growable set of characters with methods specialized to building up paths. Most of its functionality comes from the borrowed version 'Path', which is like '&str'. For instance, 'is_dir' is a 'Path' method. This might sound like a form of inheritance, but the magic Deref trait works differently. It works just like it does with 'String/&str'; a reference to 'PathBuf' can be coerced into a reference to 'Path'. 'Coerce' is a strong word, but this is one of the few places where Rust does conversions for you. */

fn foo(p: &Path) {...}
...
let path = PathBuf::from(home);
foo(&path);

/* 'PathBuf' has an intimate relationship with 'OsString' which represents strings we get directly from the system. There is also a corresponding 'OsString/&OsStr' relationship. Such strings are not guaranteed to be representable as UTF-8. There are years of ASCII legacy coding, and multiple special encodings for other languages, and human languages are complicated. 

It's true that most of the time with modern operating systems file names will be Unicode (UTF-8 on the Unix side, UTF-16 for Windows), except when they're not, and Rust must handle that possibility. For example, 'Path' has a method 'as_os_str' which returns a '&OsStr', but the 'to_str' method returns an 'Option<&str>'.

This may seem hard if you've become attached to 'string' and 'character' as the only necessary abstractions. A systems language needs a 'String/&str' distinction (owned versus borrowed: this is also very convenient) and if it wishes to standardize on Unicode strings then it needs another type to handle text which isn't valid Unicode like 'OsString/&OsStr'. There aren't any interesting string-like methods for these types because we don't know the encoding. People are used to processing filenames as if they were strings, which is why Rust makes it easier to manipulate file paths using PathBuf methods. */