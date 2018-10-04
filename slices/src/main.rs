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
    s.len() // if we don't return the full length
}
