// mod4.rs

/* The 'compilation unit' for Rust is the crate, which is either an executable or a library. To separately compile the files from the last section, first build 'foo.rs' as a Rust static library crate:

src$ rustc foo.rs --crate-type=lib
src$ ls -l libfoo.rlib
-rw-rw-r-- 1 steve steve 7888 Jan  5 13:35 libfoo.rlib

We can now link this into our main program:

src$ rustc mod4.rs --extern foo=libfoo.rlib

But the main program must now use the 'extern' name, where the 'extern' name is the same as the one used when linking. There is an implicit top-level module 'foo' associated with the library crate. */

extern crate foo;

fn main() {
    let f = foo::Foo::new("hello");
    println!("{:?}", f);
}

/* This is convenient, but there are tradeoffs. Modules are basic language features and can be used outside Cargo projects. Rust binaries are pretty big:

src$ ls -lh mod4
-rwxrwxr-x 1 steve steve 3,4M Jan  5 13:39 mod4

There is a lot of debug information in that executable. This is not bad if you want to use a debugger and have meaningful backtraces when your program panics. Let's strip that debug information:

src$ strip mod4
src$ ls -lh mod4
-rwxrwxr-x 1 steve steve 300K Jan  5 13:49 mod4

Still large for something so simple, but this program links statically against the Rust standard library. This is good since you will not need a "Rust runtime". 'rustup' will even let you cross-compile for other operating systems and platforms. We can link dynamically against the Rust runtime and get truly tiny executables:

src$ rustc -C prefer-dynamic mod4.rs --extern foo=libfoo.rlib
src$ ls -lh mod4
-rwxrwxr-x 1 steve steve 14K Jan  5 13:53 mod4
src$ ldd mod4
    linux-vdso.so.1 =>  (0x00007fffa8746000)
    libstd-b4054fae3db32020.so => not found
    libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f3cd47aa000)
    /lib64/ld-linux-x86-64.so.2 (0x00007f3cd4d72000)

That "not found" is because 'rustup' doesn't install the dynamic libraries globally. We can get aroudn this, at least on Unix (without a symlink).

src$ export LD_LIBRARY_PATH=~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib
src$ ./mod4
Foo { s: "hello" }

Rust doesn't have a philosophical problem with dynamic linking in the same way as Go does. It's just that when there's a stable release every 6 weeks it becomes inconvenient to have to recompile everything. If you have a stable version that Works For You, then cool. As stable versions of Rust get increasingly delivered by the OS package manager, dynamic linking will become more popular. */