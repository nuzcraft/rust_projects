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

    // WAYS VARIABLES AND DATA INTERACT: MOVE
    let x = 5;
    let y = x;
    // 'bind the value 5 to x; then make a copy of the value in x and bind it to y'
    // now both variables equal 5; integers are simples values with a known, fixed
    // size, and these two 5 values are pushed onto the stack.
    let s1 = String::from("hello");
    let s2 = s1;
    // a String is made up of 3 parts: a pointer to the memory that holds the contents
    // of the string, a length, and a capacity. This group is stored on the stack. The
    // contents are is memory on the heap
    // when we assign s1 to s2, we copy the pointer, length, and capacity, but not the
    // data on the heap; instead it points to the same data

    // when a variable goes out of scope, Rust calls the drop function and cleans up the
    // heap memory for that variable, but now both data pointers are pointing to the same
    // location. When s1 and s2 go out of scope, they will both try to free the same memory
    // this is known as a double free error and can cause issues

    // to ensure memory safety, rust no longer considers s1 to be valid and therefore
    // doesn't need to free anything when s1 goes out of scope
    // println!("{}, world!", s1); // this fails because the value moved to s2
    // this is known as a move. s1 was moved into s2
    // Rust will never automatically create "deep" copies of data. Any automatic
    // copying can be assumed to be inexpensive in terms of runtime performance

    // WAYS VARIABLES AND DATA INTERACT: CLONE
    // If we do want to copy the heap data, we can use a clone method
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    // clone indicates that some arbitrary code is being executed, which may be expensive

    // STACK ONLY DATA: COPY
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // types such as integers that have a known size at compile time are stored entirely
    // on the stack, so copies are quick to make. Calling clone here wouldn't do anything
    // different
    // Rust has a Copy trait that we can place on types like integers that are stored on
    // the stack. If a type has the Copy trait, an older variable is still usable after
    // assignment. rust won't let us annotate a type with the Copy trait if the type,
    // or any of its parts, has implemented the Drop trait. If the type needs something
    // special to happen when the value goes out of scope and we add the Copy annotation
    // to that type, we'll get a compile error
    // What types are Copy?
    // integers (u32, i64, etc), bool, floats (f64, etc), char, tuples that contain
    // types that are copy (i32, i32) is Copy but (i32, String) is not.

    // OWNERSHIP AND FUNCTIONS
    // passing a value to a function will move or copy, just as assignment does
    {
        let s = String::from("hello"); // s comes into scope
        takes_ownership(s); // s's value moves into the function, and is no longer valid here
        let x = 5; // x comes into scope
        makes_copy(x); // x would move into the function, but i32 is Copy, so its okay
                       // to still use x afterward
    } // here, x goes out of scope, then s. But because s's value was moved
      // nothing special happens

    // RETURN VALUES AND SCOPE
    // returning values can also transfer ownership
    {
        let s1 = gives_ownership(); // gives ownership move its return value into s1
        let s2 = String::from("hello"); // s2 comes into scope
        let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which
                                           // also moves its return value into s3
    } // here, s3 goes out of scope and is dropped. s2 goes out of scope but was moved
      // so nothing happens. s1 goes out of scope and is dropped

    // the ownership of a variable follows the same pattern every time: assigning
    // a value to another variable moves it. When the variable that includes data
    // on the heap goes out of scope, the value will be cleaned up by drop

    // taking ownership and then returning ownership with every function is a bit tedious
    //What if we want to let a function use a value but not take ownership? It’s quite 
    // annoying that anything we pass in also needs to be passed back if we want to use 
    // it again, in addition to any data resulting from the body of the function that 
    // we might want to return as well.

    // it's possible to return multiple values using a tuple
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // here, some_string goes out of scope and 'drop' is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // here, some_integer goes out of scope. Nothing special happens

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the
    // function that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function.
}

// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling fucntion
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
