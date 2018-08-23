// string4.rs (aka string2.rs)

fn main() {
    let text = "static";
    let string = "dynamic".to_string();

    // Slice notation works on strings
    let text_s = &text[1..];
    let string_s = &string[2..4];

    println!("slices {:?} {:?}", text_s, string_s);
}
// slices "tatic" "na"