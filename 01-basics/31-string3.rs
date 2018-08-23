// string3.rs (named string6.rs in tutorial)

fn array_to_str(arr: &[i32]) -> String {
    // Many types can be converted with 'to_string'
    let mut res = '['.to_string();
    for v in arr {
        // Note the '&' in front of 'v.to_string()' - the operator is defined 
        // on a string slice, not a 'String', so it needs a little persuasion.
        res += &v.to_string();
        res.push(',');
    }
    res.pop();
    res.push(']');
    res
}

fn main() {
    let arr = array_to_str(&[10, 20, 30]);
    // 'format!' is very useful for building complicated strings
    let res = format!("hello {}", arr);

    assert_eq!(res, "hello [10,20,30]");
}