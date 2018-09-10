// maps2.rs

/* A fun thing to do with text is count word frequency. It is straightforward to break text into words with 'split_whitespace', but there's also punctuation to handle, so the words should be defined as consisting only of alphabetic characters. Doing a mutable lookup on a map is straightforward, but handling the case where the lookup fails is a little awkward. Fortunately there's an elegant way to update the values of a map. */

let mut map = HashMap::new();

for s in text.split(|c: char| ! c.is_alphabetic()) {
    // convert to lowercase for easier comparison
    let word = s.to_lowercase();
    
    let mut count = map.entry(word).or_insert(0);
    *count += 1;
}

/* If there's no existing count corresponding to a word, then we'll create a new entry containing zero for that word and insert it into the map. C++ map does this as well, but it's implicit instead of Rust being explicit. There is only one explicit type in this snippet, the 'char' needed because of a quirk of the string 'Pattern' trait used by 'split'. Ee can deduce that the key type is 'String' and the value type is 'i32'. Using The Adventures of Sherlock Holmes from Project Gutenberg, we can test this out more thoroughly. The total number of unique words (map.len()) is 8071. */

// Find the twenty most common words. 
// First convert the map into a vector of (key,value) tuples. 'into_iter' consumes the map.
let mut entries: Vec<_> = map.into_iter().collect();

// Next, sort in descending order. 
// 'sort_by' expects the result of the cmp method that comes from the Ord trait, which is implemented by the integer value type:
entries.sort_by(|a,b| b.1.cmp(&a.1));

// Finally, print out the first twenty entries
for e in entries.iter().take(20) {
    println!("{} {}", e.0, e.1);
}

/* You could just loop over '0..20' and index the vector here, but it's a little un-idiomatic and potentially expensive for big iterations.

 38765
the 5810
and 3088
i 3038
to 2823
of 2778
a 2701
in 1823
that 1767
it 1749
you 1572
he 1486
was 1411
his 1159
is 1150
my 1007
have 929
with 877
as 863
had 830

What's that empty word? Split works on single-character delimiters, so any punctuation or extra spaces causes a new split. */