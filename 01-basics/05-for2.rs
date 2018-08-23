// for2.rs
fn main() {
    for i in 0..5 {
	// Modulo is just like in python
        if i % 2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i);
        }
    }
}