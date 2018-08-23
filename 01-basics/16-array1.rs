/* array1.rs
Arrays are packed nose to tail, zero indexed, as usual
They are fixed in size and mutable upon request */
fn main() {
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("first {}", first);

    for i in 0..4 {
        println!("[{}] = {}", i,arr[i]);
    }
    println!("length {}", arr.len());
}

/* Arrays are not used that often in Rust, because the type of an array includes its size.
The type of the example is [i32; 4]; the type of [10, 20] would be [i32; 2]. */