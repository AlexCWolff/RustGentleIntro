// trait2.rs (aka trait3.rs)

/* Ranges don't work for floating-point values. You can force this but you'll end up with a step of 1.0 which is useless. Recall that and iterator is a 'struct' with a 'next' method which may return 'Some' or 'None'. In the process, the iterator itself gets modified since it keeps the state for the iteration, such as the next index. The data that is being iterated over doesn't change usually (see 'Vec::drain' for an interesting iterator that does modify its data).

// The formal definition for the Iterator trait.
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    ...
}

Here we meet an associated type of the 'Iterator' trait. This trait must work for any type, so you must specify that return type without knowing it beforehand. By using an associated type, the 'next' method can be written without using a particular type, instead it refers to that type parameter's 'Item' via 'Self'. The iterator trait for 'f64' is written 'Iterator<Item=f64>', which can be read as "an Iterator with its associated type Item set to f64". The '...' refers to the provided methods of 'Iterator'. You only need to define 'Item' and 'next', and the provided methods are defined for you. */

struct FRange {
    val: f64,
    end: f64,
    incr: f64
}

fn range(x1: f64, x2: f64, skip: f64) -> FRange {
    FRange {val: x1, end: x2, incr: skip}
}

impl Iterator for FRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.val;
        if res >= self.end {
            None
        } else {
            self.val += self.incr;
            Some(res)
        }
    }
}


fn main() {
    for x in range(0.0, 1.0, 0.1) {
        println!("{} ", x);
    }
}
/*
0
0.1
0.2
0.30000000000000004
0.4
0.5
0.6
0.7
0.7999999999999999
0.8999999999999999
0.9999999999999999

This is because 0.1 is not precisely representable as a float, so a little formatting help is needed. Replace the println! with 'println!("{:.1} ", x);',
i.e. 'one decimal after dot'. All of the default iterator methods are available, so we can collect these values into a vector, map them, and so forth.

let v: Vec<f64> = range(0.0, 1.0, 0.1).map(|x| x.sin()).collect();
*/