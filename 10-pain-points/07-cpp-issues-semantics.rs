/* In C++, it's possible to define types which behave exactly like primitives and copy themselves. In addition, a move constructor can be defined to specify how a value can be moved out of a temporary context. In Rust, primitives behave as expected, but the 'Copy' trait can only be defined if the aggregate type (struct, tuple or enum) itself contains only copyable types. Arbitrary types may have 'Clone', but you have to call the 'clone' method on values. Rust requires any allocation to be explicit and not hide in copy constructors or assignment operators.

Copying and moving is always defined as just moving bits around and is not overrideable. If 's1' is a non 'Copy' value type, then 's2 = s1;' causes a move to happen, and this consumes 's1', so when you really want a copy, use 'clone'.

Borrowing is often better than copying, but then you must follow the rules of borrowing. Fortunately, borrowing is an overridable behaviour. For instance, 'String' can be borrowed as '&str', and shares all the immutable methods of '&str'. String slices are very powerful compared to the analogous C++ borrowing operation, which is to extract a 'const char*' using 'c_str'. '&str' consists of a pointer to some owned bytes (or a string literal) and a size. This leads to some very memory-efficient patterns. You can have a 'Vec<&str>' where all the strings have been borrowed from some underlying string - only space for the vector needs to be allocated. */

// For example, splitting by whitespace:
fn split_whitespace(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}
// Likewise, a C++ 's.substr(0,2)' call will always copy the string, but a slice will just borrow: '&s[0..2]'. There is an equivalent relationship between 'Vec<T>' and '&[T]'.