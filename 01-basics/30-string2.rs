// string2.rs (named string5.rs in tutorial)

fn main() {
    let mut s = String::new();
    // Like a vector, you can push and pop the 'String'
    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!"; // short for `push_str`
    // remove the last char
    s.pop();

    assert_eq!(s, "Hello World");
}
