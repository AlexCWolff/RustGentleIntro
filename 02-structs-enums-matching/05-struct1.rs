// struct1.rs

// Rust structs contain named fields
struct Person {
    first_name: String,
    last_name: String
}

fn main() {
    let p = Person {
        first_name: "John".to_string(),
        last_name: "Smith".to_string()
    };
    println!("person {} {}", p.first_name,p.last_name);
}

/* The values of a struct will be placed next to each other in memory, but you 
should not assume memory layout. The compiler will organize the memory for 
efficiency, so there may be padding. */