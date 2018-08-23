// slice3.rs (named slice4.rs in tutorial)

/* Messing with some slice methods now.
https://doc.rust-lang.org/std/primitive.slice.html
The windows method gives you an iterator of slices: overlapping windows of values */

fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;

    for s in slice.windows(2) {
        println!("window {:?}", s);
    }
  
    for s in slice.chunks(2) {
        println!("chunks {:?}", s);
    }
}
/* 
window [1, 2]
window [2, 3]
window [3, 4]
window [4, 5]
chunks [1, 2]
chunks [3, 4]
chunks [5] 
*/