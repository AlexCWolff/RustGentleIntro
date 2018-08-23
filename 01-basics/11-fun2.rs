// fun2.rs

// Functions with return statements are rare because the body of the function has the value of its last expression
// return is still useful for returning early from a function
fn main() {
    let res = sqr(2.0);
    println!("square is {}", res);
}

// absolute value of a floating-point number
fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

// ensure the number always falls in the given range
fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}

