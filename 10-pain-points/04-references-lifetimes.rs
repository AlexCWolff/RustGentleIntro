/* Rust cannot allow a situation where a reference outlives the value. Otherwise we would have a 'dangling reference' where it refers to a dead value - a segfault is inevitable.

rustc can often make sensible assumptions about lifetimes in functions:

fn pair(s: &str, ch: char) -> (&str, &str) {
    if let Some(idx) = s.find(ch) {
        (&s[0..idx], &s[idx+1..])
    } else {
        (s, "")
    }
}
fn main() {
    let p = pair("hello:dolly", ':');
    println!("{:?}", p);
}
// ("hello", "dolly")
This is quite safe because we cope with the case where the delimiter isn't found. rustc is here assuming that both strings in the tuple are borrowed from the string passed as an argument to the function.

Explicitly, the function definition looks like this:

fn pair<'a>(s: &'a str, ch: char) -> (&'a str, &'a str) {...}
What the notation says is that the output strings live at least as long as the input string. It's not saying that the lifetimes are the same, we could drop them at any time, just that they cannot outlive s.

So, rustc makes common cases prettier with lifetime ellision.

Now, if that function received two strings, then you would need to explicitly do lifetime annotation to tell Rust what output string is borrowed from what input string.

You always need an explicit lifetime when a struct borrows a reference:

struct Container<'a> {
    s: &'a str
}
Which is again insisting that the struct cannot outlive the reference. For both structs and functions, the lifetime needs to be declared in <> like a type parameter.

Closures are very convenient and a powerful feature - a lot of the power of Rust iterators comes from them. But if you store them, you have to specify a lifetime. This is because basically a closure is a generated struct that can be called, and that by default borrows its environment. Here the linear closure has immutable references to m and c.

let m = 2.0;
let c = 0.5;

let linear = |x| m*x + c;
let sc = |x| m*x.cos()
...
Both linear and sc implement Fn(x: f64)->f64 but they are not the same animal - they have different types and sizes! So to store them you have to make a Box<Fn(x: f64)->f64 + 'a>.

Very irritating if you're used to how fluent closures are in Javascript or Lua, but C++ does a similar thing to Rust and needs std::function to store different closures, taking a little penalty for the virtual call. */