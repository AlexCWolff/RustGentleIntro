// docs.rs

/* Reading the Rust documentation can be challenging, so lets go through 'Vec' as an example. A useful tip is to tick the "[-]" box to collapse the docs, this gives you a bird's eye view of all the available methods. The first point to notice is that not all possible methods are defined on 'Vec' itself. They are mostly mutable methods that change the vector, like 'push'. Some methods are only implemented for vectors where the type matches some constraint. For example, you can only call 'dedup' if the type is something that can be compared for equality. There are multiple 'impl' blocks that define 'Vec' for different type constraints.

Then there's the special relationship between 'Vec<T>' and '&[T]'. Any method that works on slices will also directly work on vectors, without explicitly having to use the 'as_slice' method. This relationship is expressed by 'Deref<Target=[T]>'. This also kicks in when you pass a vector by reference to something that expects a slice - this is one of the few places where a conversion between types happens automatically. So slice methods like 'first', which maybe-returns a reference to the first element, or 'last', work for vectors as well. Many of the methods are similar to the corresponding string methods, so there's 'split_at' for getting a pair of slices split at an index, 'starts_with' to check whether a vector starts with sequence of values, and 'contains' to check whether a vector contains a particular value.

There's no 'search' method for finding the index of a particular value, but if you can't find a method on the container, look for a method on the iterator: */

let v = vec![10,20,30,40,50];
// The '&' is because this is an iterator over references. You could also say '*i == 30'.
assert_eq!(v.iter().position(|&i| i == 30).unwrap(), 2);
    
/* There's no 'map' method on vectors because 'iter().map(...).collect()' will do. You usually don't need the result of that 'map' as an actual allocated vector. Become familiar with all the iterator methods, they are crucial to writing good code without loops. 

The 'Vec<T>' and '&[T]' methods are followed by the common traits. Vectors know how to do a debug display of themselves, but only if the elements implement 'Debug'. Likewise, they are clonable if their elements are clonable. They implement 'Drop', which happens when vectors get to finally die; memory is released, and all the elements are dropped as well. The 'Extend' trait means values from iterators can be added to a vector without a loop. */

v.extend([60,70,80].iter());
let mut strings = vec!["hello".to_string(), "dolly".to_string()];
strings.extend(["you","are","fine"].iter().map(|s| s.to_string()));

/* 'FromIterator' lets vectors be constructed from iterators. The iterator collect method relies on this. Any container needs to be iterable as well. Recall that there are three kinds of iterators: */

for x in v {...} // returns T, consumes v
for x in &v {...} // returns &T
for x in &mut v {...} // returns &mut T

/* The 'for' statement relies on the 'IntoIterator' trait, and there's three implementations. Next there is indexing, which is controlled by 'Index' (reading from a vector) and 'IndexMut' (modifying a vector). There are many possibilities because of slice indexing, which also return slices, as well as 'v[0]' which returns a reference to the first element.

There's a few implementations of the 'From' trait, for instance 'Vec::from("hello".to_string())' will give you a vector of the underlying bytes of the string 'Vec<u8>'. There's already a method 'into_bytes' on 'String', so why the redundancy? It's needed because explicit traits make generic methods possible. Sometimes the limitations of the Rust type system make things clumsy, like how 'PartialEq' is separately defined for arrays up to size 32. This allows the convenience of directly comparing vectors with arrays, but beware the size limit. */

// There are hidden gems buried deep in the documentation. How do you handle errors in an iterator?
// 'map' over some operation that might fail, so they return 'Result', then collect the results
fn main() {
    let nums = ["5","52","65"];
    let iter = nums.iter().map(|s| s.parse::<i32>());
    let converted: Vec<_> = iter.collect();
    println!("{:?}",converted);
}
//[Ok(5), Ok(52), Ok(65)]

/* Now you have to unwrap these errors. Rust already knows how to do the right thing, if you ask for the vector to be contained in a 'Result', i.e. either is a vector or an error: */

let converted: Result<Vec<_>,_> = iter.collect();
//Ok([5, 52, 65])

/* If there is a bad conversion you would just get 'Err' with the first error encountered. It's a good example of how extremely flexible 'collect' is. The notation can be intimidating; 'Vec<_>' means "this is a vector, work out the actual type for me and return a 'Result'". There's a lot of detail in the documentation, but it's much clearer than the equivalant C++ docs.

The requirements that are imposed on the elements depend on the actual operations performed on the container. Generally, it is required that element type is a complete type and meets the requirements of Erasable, but many member functions impose stricter requirements.

The explicitness of Rust is daunting at first, but as you learn to read the constraints you will know exactly what any particular method of Vec requires. With C++, you're on your own. I would suggest that you get the source using 'rustup component add rust-src', since the standard library source is very readable and the method implementations are usually less scary than the method declarations. */