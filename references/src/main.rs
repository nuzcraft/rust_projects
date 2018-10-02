fn main() {
    // referencing the code in the 'ownership' project...
    // let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    // println!("The length of '{}' is {}.", s2, len);

    // fn calculate_length(s: String) -> (String, usize) {
    // let length = s.len(); // len() returns the length of a String
    // (s, length)
    // }

    // the issue here is that we have tor return the string back to the calling function
    // so we can continue to use it after moving it into the calculate_length function
    // below is an example of a function that has a reference to an object as a parameter
    // instead of taking ownership of the value
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    // the '&'s are references, and they allow us to refer to a value without taking
    // ownership of it. in this example, s points to s1 which points to the value on
    // the heap

    // having references as function parameters is called borrowing
    // you can't modify something that you're borrowing
    let s = String::from("hello");
    change(&s);

    // just as variables are immutable by default, so are references
    // we can change the above code to use a mutable reference instead
    let mut s = String::from("hello");
    change_mut(&mut s);
    // first, we had to change s to be mutable, then the reference had to be mutable
    // as well. Restriction: only one mutable reference to a piece of data is
    // allowed an a particular scope: the following will fail
    // let r1 = &mut s;
    // let r2 = &mut s;
    // this keeps us from try to access and change the same piece of data
    // all at once
    // note: we can have multiple mutable reference, just not in the same scope
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    // a similar rule exists for combining mutable and immutable references
    let mut t = String::from("hello");
    let r1 = &t; // no problem
    let r2 = &t; // no problem
                 // let r3 = &mut t; // BIG PROBLEM
                 // we also can't have a mutable reference while we have an immutable one
                 // again to keep us from accessing and changing data at the same time

    //DANGLING REFERENCES
    // in languages with pointers, it can be easy to erroneously create a dangling
    // pointer, a pointer that references a location in memory that may have been
    // given to someone else, by freeing some memory, but preserving the pointer
    // Rust's compiler makes sure the references goes out of scope before the data
    // let reference_to_nothing = dangle();
    // this will fail because 'the function's return type contains a borrowed
    // value, but there is no value to borrow from
    // the solution is to return the String directly
    let reference_to_somethign = no_dangle();
    // ownership is moved out, and nothing is deallocated

    //* at any given time, you can have either (but not both of) one mutable reference
    //  or any number of immutable references
    //* references must always be valid
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // here, s goes out of scope. But becasue it does not have ownership of what it refers
  // to, nothing happens

fn change(some_string: &String) {
    // some_string.push_str(", world"); // this won't work because its borrowing
}

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello"); // s is a new String
//     &s // we return a reference to the String, s
// } // here, s goes out of scope, and is dropped. It's memory goes away. Danger!

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
