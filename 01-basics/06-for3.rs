// for3.rs
fn main() {
    for i in 0..5 {
	// No brackets around the condition, just curly braces around the block
	// In Rust, nearly everything has a value and can be an expression
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);
    }
}