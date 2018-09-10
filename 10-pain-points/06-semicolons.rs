/* Semicolons are not optional, but usually left out in the same places as in C, e.g. after {} blocks. They also aren't needed after enum or struct (that's a C peculiarity.) However, if the block must have a value, then the semi-colons are dropped:

    let msg = if ok {"ok"} else {"error"};
Note that there must be a semi-colon after this let statement!

If there were semicolons after these string literals then the returned value would be () (like Nothing or void). It's common error when defining functions:

fn sqr(x: f64) -> f64 {
    x * x;
}
rustc will give you a clear error in this case. */