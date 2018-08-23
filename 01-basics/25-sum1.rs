// sum1.rs

/* Previously, we did a sum of a range of integers using mutable variables. 
Here's the idiomatic way of doing this */
fn main() {
    // Must be explicit about type, can't be inferred 
    let sum: i32  = (0..5).sum();
    println!("sum was {}", sum);

    let sum: i64 = [10, 20, 30].iter().sum();
    println!("sum was {}", sum);
}