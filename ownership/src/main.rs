fn main() {
    // rust doesn't have a garbage collector, so it relies on ownership
    // to manage memory correctly

    //OWNERSHIP RULES
    // 1. Each value in Rust has a variable that's called its owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped

    // Variable scope
    {
        // s is not valid here, it's not yet declared
        let s = "hello"; // s is valid from this point forward
    } // this scope is now over, and s is no longer valid
      // When s comes into scope, it is valid. It remains valid until it goes out of scope.

    // The String Type
    // we've already seen string literals (hardcoded like s="hello"), but they aren't
    // always suitable, they are immutable.
    // a second string type, String. Allocated on the heap, and can store an amount
    // of text that is unknown at compile time
    let mut s = String::from("hello");
    // :: allows us to namespace this particular from function under the String type
    // this type of string can be mutated
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print 'hello, world!'
                       // String can be mutated because it deals with memory differently than a string literal

    // MEMORY AND ALLOCATION
    // for string literals, we know the contents at compile, so the text is hardcoded
    // with the String type, in order to suport a mutable piece of text, we need to
    // allocate some amount of memory on the heap (unknown at compile time), to hold it
    // * memory must be requested from the operating system at runtime
    // ** this is done by us with String::from
    // * we need a way of returning this memory to the operating system when we're done
    // ** this part is difficult
    // other languages have a garbage collector (gc) that keeps track of and cleans up
    // memory that isn't being used anymore. without a gc, its our responsibility to
    // identify when memory is no longer being used and call code to return it
    // if we forget, we waste memory. if we do it too early, we'll have an invalid variable
    // if we do it twice, thats a bug. we need ONE allocate and ONE free
    // Rust automatically returns memory once the variable goes out of scope
    {
        let s = String::from("hello"); // s is valid from this point forward
                                       // do stuff with s
    } // this scope is now over, and s is no longer valid
      // Rust has a drop function that is called automatically at the closing }
}
