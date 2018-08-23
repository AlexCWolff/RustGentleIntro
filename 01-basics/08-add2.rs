// add2.rs
fn main() {
    // 'mut' asks to make your variable mutable
    let mut sum = 0;
    for i in 0..5 {
        sum += i;
    }
    println!("sum is {}", sum);
}