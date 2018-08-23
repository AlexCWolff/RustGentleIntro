// add1.rs
fn main() {
    let sum = 0;
    for i in 0..5 {
        // Will not compile, variables are immutable by default
	sum += i;
    }
    println!("sum is {}", sum);
}