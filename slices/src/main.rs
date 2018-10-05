fn main() {
    // another data type that does not have ownership is a slice
    // slices let you reference a contiguous sequence of elements in a collection
    // rather than the whole collection

    // thinking about the following problem: write a function that takes a string and
    // returns the first word it finds in that string. If the function doesn't find a 
    // space in the string, the whole string must be one word, so the entire string
    // should be returned
    // fn first_word(s: &String) -> ? // what do we return for part of a string?
    // lets try returning the index of the end of the word

    // we have a way to find the index of the end of the first word in the string, but...
    // that number is only meaningful in the context of &String
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that we could 
    // meaningfully use the value 5 with. word is now totally invalid!

    // having to worry about the index in word geting out of sync with data in s
    // is a pain. Rust's solution is string slices

    // a string slice is a reference to part of a STring
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    // this is really similar to taking a reference to the whole string, but adding [0..5]
    // the start..end syntax is a range that begins with start and continues up to
    // but not including end. You can use ..= instead of .. to make it end inclusive
    let hello = &s[0..=4];
    let world = &s[6..=10];
    // in memory, these are stored as pointers to the starting index and a length

    // with Rust's range syntax, you can drop the 0 if you want to start at the beginning
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];
    // or you can drop both values to take a slice of the entire string (these are equal)
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];

    // lets rewrite first_word (to new_first_word) so that it returns a string slice
    // instead of an index. Since the returned first word is a string slice, it references
    // the original String
    let mut s = String::from("hello world");
    let word = new_first_word(&s);
    // s.clear(); // this throws an error because word is borrowing from s

    //STRING LITERALS ARE SLICES
    let s = "Hello, world!";
    // The type of s is &str: it's a slice pointing to that specific point of the binary
    // this is also why string literals are immutable; &str is an immutable reference

    //STRING SLICES AS PARAMETERS
    // change a functions from:
    // fn first_word(s: &String) -> &str {
    // to
    // fn first_word(s: &str) -> &str {
    // if we have a string slice, we can pass it directly. If we have a String, we can 
    // pass it a string slice of the whole String. This is a generalization that doesn't
    // lose any functionality, but adds additional usefulness
    let my_string = String::from("hello world");
    // new first word works on slices of Strings
    let word = new_first_word(&my_string[..]);
    let my_string_literal = "hello world";
    // new first word works on slices of string literals
    let word = new_first_word(&my_string_literal[..]);
    // beause string literals are string slices already, this works too,
    // without the slice syntax
    let word = new_first_word(my_string_literal);

    //OTHER SLICES
    // string slices are specific to strings, but there's a more general slice type
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    // slice has type &[i32], and works the same way as string slices do

}

// this function returns a byte index value into the String parameter
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert our String into an array of bytes

    for (i, &item) in bytes.iter().enumerate() { // iterate over the array of bytes
    // iter() returns each element in a collection and enumerate() wraps the result
    // and returns each element as part of a tuple (index, reference to element)
    // this is demonstrated by (i, &item) in the destructure pattern of the for
        if item == b' ' { // using byte literal syntax to search for the space
            return i; // if we find it, return the index
        }
    }
    s.len() // if we don't find a space, return the full length
}

// rewriting first_word to return a string slice instead of an index
// EDIT: (s: &String) changed to (s: &str)
fn new_first_word(s: &str) -> &str { // &str indicates a string slice
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
