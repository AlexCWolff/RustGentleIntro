// enum2.rs

// Rust does 'C style enums' as well
enum Speed {
    Slow = 10,
    Medium = 20,
    Fast = 50
}

fn main() {
    let s = Speed::Slow;
    let speed = s as u32;
    println!("speed {}", speed);
}

/*They are initialized with an integer value, and can be converted into that integer with a type cast. You only need to give the first name a value, after that the value increments by one */
enum Difficulty {
    Easy = 1,
    Medium,  // is 2
    Hard   // is 3
}

/* The proper term for 'name' here is variant; 'Speed' has variants 'Slow', 'Medium', and 'Fast'. These enums do have a natural ordering, but you have to ask. After placing '#[derive(PartialEq,PartialOrd)]' in front of 'enum Speed', then it's true that 'Speed::Fast > Speed::Slow' and 'Speed::Medium != Speed::Slow'. etc. */