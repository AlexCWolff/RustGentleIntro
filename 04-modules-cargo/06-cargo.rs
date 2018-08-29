// cargo.rs

/* The Rust standard library is not very large compared to Java or Python; although much more fully featured than C or C++ which lean heavily on operating system provided libraries. It is straightforward to access community-provided libraries in crates.io using Cargo. Cargo will look up the correct version and download the source for you, and ensures that any other needed crates are downloaded as well. 

Initialize a Cargo project using '--bin' because the default is to create a library project:

test$ cargo init --bin test-json
     Created binary (application) project
test$ cd test-json

To make the project depend on the JSON crate, edit the 'Cargo.toml' file:

[dependencies]
json="0.11.4"
Then do a first build with Cargo:

test-json$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading json v0.11.4
   Compiling json v0.11.4
   Compiling test-json v0.1.0 (file:///home/steve/c/rust/test/test-json)
    Finished debug [unoptimized + debuginfo] target(s) in 1.75 secs
    
GOTO: test-json/src/main.rs */