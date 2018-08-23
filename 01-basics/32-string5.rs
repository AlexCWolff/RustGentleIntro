// string5.rs (aka string 3.rs)

/* You cannot index strings since they are UTF-8 and a character might be a 
number of bytes */
fn main() {
    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() {
        print!("'{}' ", ch);
    }
    println!("");
    println!("len {}", multilingual.len());
    println!("count {}", multilingual.chars().count());

    let maybe = multilingual.find('п');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi {}", hi);
    }
}
/* 'H' 'i' '!' ' ' '¡' 'H' 'o' 'l' 'a' '!' ' ' 'п' 'р' 'и' 'в' 'е' 'т' '!'
len 25
count 18
Russian hi привет!

Noticed there's 25 bytes but 18 characters. If you use a method like 'find', 
you will get a valid index (if found) and any slice will be fine. Note that
'char' type is a 4-byte Unicode code point, Strings are not arrays of chars.

String slicing may explode like vector indexing, because it uses byte offsets. 
In this case, the string consists of two bytes, so trying to pull out the 
first byte is a Unicode error. Be careful to only slice strings using valid 
offsets that come from string methods.

let s = "¡";
println!("{}", &s[0..1]); <-- bad, first byte of a multibyte character

Breaking up strings is very common; the string 'split_whitespace' method returns 
an iterator, and we then choose what to do with it. Commonly, to create a 
vector of the split substrings. 

'collect' is very general and so needs some clues about what it is collecting - hence the explicit type.

let text = "the red fox and the lazy dog";
let words: Vec<&str> = text.split_whitespace().collect();
// ["the", "red", "fox", "and", "the", "lazy", "dog"]

You could also say it like this, passing the iterator into the 'extend' method:

let mut words = Vec::new();
words.extend(text.split_whitespace());

Here each slice in the vector is borrowing from the original string, all we allocate is the space to keep the slices. In the next example, we get an iterator over the chars and only take those characters which are not space.

let stripped: String = text.chars()
    .filter(|ch| ! ch.is_whitespace()).collect();
// theredfoxandthelazydog

The filter method takes a closure, i.e. lambdas. The argument type is clear from
the context, so the explicit rule is relaxed. Yes, you can do this as an explicit 
loop over chars, pushing the returned slices into a mutable vector, but this is 
more concise and just as fast.
*/