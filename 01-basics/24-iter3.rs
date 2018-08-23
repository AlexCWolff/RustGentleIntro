// iter3.rs

// The correct version, based on compiler feedback.
fn main() {
    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("{}", i);
    }

    // slices will be converted implicitly to iterators...
    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }
}

/* It's more efficient to iterate over an array or slice this way than to use 
'for i in 0..slice.len() {}' so it does'nt have to check every index operation.*/