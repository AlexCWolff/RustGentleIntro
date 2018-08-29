// test-json/src/main.rs

extern crate json;

fn main() {
    // Note the convenient 'raw' string literal, otherwise we would need to escape those double quotes
    let doc = json::parse(r#"
    {
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    }
    "#).expect("parse failed");

    println!("debug {:?}", doc);
    println!("display {}", doc);
}

/* You can now build and run this project - only main.rs has changed.

test-json$ cargo run
   Compiling test-json v0.1.0 (file:///home/steve/c/rust/test/test-json)
    Finished debug [unoptimized + debuginfo] target(s) in 0.21 secs
     Running `target/debug/test-json`
debug Object(Object { store: [("code", Number(Number { category: 1, exponent: 0, mantissa: 200 }),
 0, 1), ("success", Boolean(true), 0, 2), ("payload", Object(Object { store: [("features",
 Array([Short("awesome"), Short("easyAPI"), Short("lowLearningCurve")]), 0, 0)] }), 0, 0)] })
display {"code":200,"success":true,"payload":{"features":["awesome","easyAPI","lowLearningCurve"]}}

The debug output shows some internal details of the JSON document, but a plain '{}', using the 'Display' trait regenerates JSON from the parsed document. It would not be useful if we could not extract values. The 'as_TYPE' methods return 'Option<TYPE>' since we cannot be sure that the field exists or is of the correct type. (see the docs for JsonValue)

    let code = doc["code"].as_u32().unwrap_or(0);
    let success = doc["success"].as_bool().unwrap_or(false);

    assert_eq!(code, 200);
    assert_eq!(success, true);

    // 'features' is a reference to 'JsonValue'; it has to be a reference otherwise we would be trying to move a value out of the JSON document
    let features = &doc["payload"]["features"];
    // We know it's an array, so 'members()' will return a non-empty iterator over '&JsonValue'
    for v in features.members() {
        println!("{}", v.as_str().unwrap()); // MIGHT explode
    }
    // awesome
    // easyAPI
    // lowLearningCurve
 
If the 'payload' object didn't have a 'features' key then 'features' would be set to 'Null'. There will be no explosion thanks to the free-form nature of JSON. It is up to you to examine the structure of any document you receive and create your own errors if the structure does not match. You can modify these structures. If we had 'let mut doc' then this would do what you expect:

let features = &mut doc["payload"]["features"];
features.push("cargo!").expect("couldn't push");

The 'push' will fail if 'features' wasn't an array, hence it returns 'Result<()>'.

Here's using macros to generate JSON literals:

let data = object!{
    "name"    => "John Doe",
    "age"     => 30,
    "numbers" => array![10,53,553]
};
assert_eq!(
    data.dump(),
    r#"{"name":"John Doe","age":30,"numbers":[10,53,553]}"#
);

For this to work you need to explicitly import macros from the JSON crate:

#[macro_use]
extern crate json;

There is a downside to using this crate because of the mismatch between the amorphous, dynamically-typed nature of JSON and the structured, static nature of Rust. If you did want to map JSON to Rust data structures, you would end up doing a lot of checking because you can not assume that the received structure matches your structs. A better solution is serde_json where you serialize Rust data structures into JSON and deserialize JSON into Rust. 

GOTO: test-serde-json/src/main.rs */