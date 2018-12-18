fn main() {
    println!("Hello, world!");
    // STORING LISTS OF VALUES WITH VECTORS
    // Vec<T> is a collection type known as a vector. They allow you to
    // store more than one value in a single data structure that puts all
    // the values next to each other in memory.
    //
    // Vectors can only store values of the same type. Useful for lists
    // of items; lines of text or prices of items in a cart for example
    //
    // Use Vec:: new to create a new, empty vector
    let v: Vec<i32> = Vec::new();
    // notice the type annotation Vec<T>. Because the vector is empty,
    // we need to tell rust what kind of elements we want to store.
    // 
    // Vectors are implemented using generics. Vec<T> can hold any type,
    // and when a vector holds a specific type, the type is specified
    // in the angle brackets.
    // The vector above, v, will hold i32 type elements
    //
    // In 'real' code, Rust can often infer the types of value once inserted
    // type annotation is often not needed. Creating a vector with initial
    // values is common, and Rust provides the vec! macro to do just that
    let v = vec![1, 2, 3];
    // because we've given initial i32 values, Rusn can inver that the type
    // of v is Vec<i32> and the type annotation is unnecessary.
    //
    // UPDATING A VECTOR
    // use the push method to add elements to a created vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    // mut means the variable is mutable (required if we want to add values to it)
    // even though we push values into v after instantiation, Rust can infer type
    // i32 for v
    //
    // DROPPING A VECTOR DROPS ITS ELEMENTS
    // like other structs, a vector is freed when it goes out of scope
    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here
    // when a vector is dropped, so are its contents. This can cause issues when
    // elements of a vector are being referenced
    //
    // READING ELEMENTS OF VECTORS
    // you can read from vectors in a couple ways:
    // indexing syntax to access the value of a vector
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    // get method to access the value of a vector
    let v = vec![1, 2, 3, 4, 5];
    let v_index = 2;
    match v.get(v_index) {
        Some(_) => {println!("Reachable element at index: {}", v_index)},
        None => {println!("Unreachable element at index: {}", v_index)},
    }
    // details: we use index 2 to get the third element, as indexes start at 0
    // the & and [] in the indexing syntax method mean we are using a reference
    // the get method uses an index passed as an argument, which returns an
    // Option<&T>
    // Both options are available so you can choose how the program reacts when
    // trying to use an index value that the vector doesn't have an element for
    //
    // let does_not_exist = &v[100]; // this causes a panic
    // let does_not_exist = v.get(100);
    // the first method causes the program to panic bc it references a nonexistent
    // element. This method is best used when you want your program to crash is 
    // there's an attempt to access an element past the end of the vector
    //
    // the get method will return None without panicking. your code should then have
    // logic to handle this situation if it occurs occasionally. More user friendly
    //
    // When the program has a valid reference, the borrow checker enforces ownership
    // and borrowing rules. Recall the rule that states you can't have mutable and immutable
    // references in the same scope
    // let mut v = vec![1, 2, 3, 4, 5]; // mutable vector
    // let first = &v[0]; // immutable borrow to first element in the vector
    // v.push(6); // mutable borrow to add an element to the end (fails)
    // println!("The first element is: {}", first); // borrow used here later
    //
    // this might seem odd: why should a reference to the first element care about what
    // changes at the end of the vector? adding a new element may require allocating new 
    // memory and copying the old elements to a new space. If this happens, the first
    // element would be pointing at deallocated memory
    //
    // ITERATING OVER THE VALUES IN A VECTOR
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    // this is a for loop to get immutable references to each element in a vector
    //
    // we can also iterate over mutable references to each element in a mutable vector
    // in order to make changes to all the elements
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    // to change the value the mutable reference refers to, we have to use the
    // dereference operator (*) to get to the value in i before we can use the 
    // (+=) operator. We'll hit * in a later chapter.
    //
    // USING AN ENUM TO STORE MULTIPLE TYPES
    // vectors can only store values that are the same type, which can be inconvenient
    // fortunately, the variants of an enum are defined under the same enum type, so
    // when we need to store elements of a different type in a vector, we can define
    // and use an enum
    //
    // for example, if we want to store a row from a spreadsheet in a vector, we can
    // define an enum whose variants hold those types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // rust needs to know what types will be in the vector at compile time so it knows how
    // much memory on the heap will be needed to store each element. This also reduces the
    // possibility of errors when performing operations on a vector, because we always know
    // what type the values will be.
}
