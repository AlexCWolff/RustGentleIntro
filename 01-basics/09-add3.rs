// add3.rs
fn main() {
    let mut sum = 0.0;
    for i in 0..5 {
	// Rust will not implicitly convert, you must cast it yourself
        sum += i as f64;
    }
    println!("sum is {}", sum);
}