// mod3.rs

// Use a 'mod foo' statement without a block in the main program to use the foo code.
// The compiler will also look at MODNAME/mod.rs
// The main program can use both modules as separate files
mod foo;
mod boo;

fn main() {
    let f = foo::Foo::new("hello");
    let res = boo::answer();
    println!("{:?} {}", f,res);
}

/* Now 'rustc mod3.rs' will cause the module files to be compiled as well, no need for makefiles. So far there's 'mod3.rs' which contains main, a module 'foo.rs', and a directory 'boo' containing 'mod.rs'. The usual convention is that the file containing 'main' is just called 'main.rs'. Why two ways to do the same thing? Because 'boo/mod.rs' can refer to other modules defined in 'boo', update 'boo/mod.rs', and add a new module. Note that this is explicitly exported. Without the 'pub', 'bar' can only be seen inside the 'boo' module. 

Modules are about organization and visibility, and this may or may not involve separate files. Note that 'use' has nothing to do with importing, it simply specifies visibility of module names. For example:

{
    use boo::bar;
    let q = bar::question();
    ...
}
{
    use boo::bar::question();
    let q = question();
    ...
}

There is no separate compilation here, the main program and its module files will be recompiled each time. Larger programs will take a fair amount of time to build, although 'rustc' is getting better at incremental compilation. */