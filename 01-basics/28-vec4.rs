// vec4.rs

/* Vectors compare with each other and with slices by value. You can insert and
remove values into a vector with 'insert' and 'remove', but this is not as 
efficient as pushing and popping since the values will have to be moved. Vectors 
have a size and a capacity, if you clear a vector its size becomes zero but it
still retains its old capacity. Vectors can be sorted, and then duplicates can 
be removed. These operate in-place on the vector. If you want to make a copy 
first, use 'clone'.
*/

fn main() {
    let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    v1.sort();
    v1.dedup();
    assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);
}