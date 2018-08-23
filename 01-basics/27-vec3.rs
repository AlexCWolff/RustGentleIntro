// vec3.rs

fn main() {
    // vec! will initialize a vector for you
    let mut v1 = vec![10, 20, 30, 40];
    // pops the last value, as you'd expect
    v1.pop();

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);

    assert_eq!(v1, v2);

    // Vectors can be extended with any compatible iterator
    v2.extend(0..2);
    assert_eq!(v2, &[10, 20, 30, 0, 1]);
}