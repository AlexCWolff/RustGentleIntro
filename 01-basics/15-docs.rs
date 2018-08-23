// docs.rs

/* Docs are installed with Rust
Run 'rustup doc --std' to open them in a browser
For example, the mathematical function for cosine
The type must be explicit, without it the constant could be either f32 or f64

let pi: f64 = 3.1416;
let x = pi/2.0;
let cosine = x.cos(); */

/* Cosine example as a complete program, using fully qualified name
 '::' is just like in C++, or '.' in other languages
 
fn main() {
    let x = 2.0 * std::f64::consts::PI;
    let abs_difference = (x.cos() - 1.0).abs();
    assert!(abs_difference < 1e-10);
} */

// 'use' is the same as 'import', 'include', etc. in other languages
use std::f64::consts;

fn main() {
    let x = 2.0 * consts::PI;
    let abs_difference = (x.cos() - 1.0).abs();
    assert!(abs_difference < 1e-10);
}