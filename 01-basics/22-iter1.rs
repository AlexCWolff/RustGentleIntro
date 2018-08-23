// iter1.rs

/* Informal version, just like 'range' in Python 3.
It is an 'object' with a next method which returns an Option. 
As long as that value is not None, we can keep calling next.
*/
fn main() {
    let mut iter = 0..3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
}