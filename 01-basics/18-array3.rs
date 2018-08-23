// array3.rs

/* You cannot print out an array with {}, but you can do a debug print with {:?}
Arrays of arrays are fine, but an array must only contain values of one type. */
fn main() {
    let ints = [1, 2, 3];
    let floats = [1.1, 2.1, 3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1, 2], [10, 20]];
    println!("ints {:?}", ints);
    println!("floats {:?}", floats);
    println!("strings {:?}", strings);
    println!("ints_ints {:?}", ints_ints);
}

/* If you are curious about the actual types of these variables,  
just declare a variable with an explicit type which you know will be wrong:

let var: () = [1.1, 1.2];

3 |     let var: () = [1.1, 1.2];
  |                   ^^^^^^^^^^ expected (), found array of 2 elements
  |
  = note: expected type `()`
  = note:    found type `[{float}; 2]`
  
  {float} means 'some floating-point type which is not fully specified yet' */