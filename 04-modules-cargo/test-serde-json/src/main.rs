// test-serde-json/src/main.rs

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    address: Address,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    city: String,
}

fn main() {
    let data = r#" {
     "name": "John Doe", "age": 43,
     "address": {"street": "main", "city":"Downtown"},
     "phones":["27726550023"]
    } "#;
    let p: Person = serde_json::from_str(data).expect("deserialize error");
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    println!("{:#?}",p);
}

/* The 'serde_derive' crate defines custom derives for the special 'Serialize' and 'Deserialize' traits. 

Please call John Doe at the number 27726550023
Person {
    name: "John Doe",
    age: 43,
    address: Address {
        street: "main",
        city: "Downtown"
    },
    phones: [
        "27726550023"
    ]
}

If you did this using the 'json' crate you would need a few hundred lines of custom conversion code, mostly error handling. This is the best solution if you are processing well-structured JSON from outside sources (it's possible to remap field names if needed) and provides a robust way for Rust programs to share data with other programs over the network since everything understands JSON these days. The nice thing about 'serde' (SERialization DEserialization) is that other file formats are also supported, such as 'toml', so your program can read .toml files into structs and write those structs out as .json. */