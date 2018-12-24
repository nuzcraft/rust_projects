#[allow(unused_variables)]
#[allow(unused_mut)]
fn main() {
    println!("Hello, world!");
    // STORING UTF-8 ENCODED TEXT WITH STRINGS
    // strings are tough (though common). strings are implemented as a collection of bytes
    // plus some methods for when they are interpreted as text

    // WHAT IS A STRING?
    // Rust has only one string type in the core language, which is the string slice str
    // that is usually seen in its borrowed form &str

    // The String type, provided by the standard library rather than coded into the core 
    // language, is a growable, mutable, owned, UTF-8 encoded string type.AsMut

    // Rust's standard library also includes a number of other string types: OsString, 
    // OsStr, CString, and CStr. Library crates can provide even more options for storing
    // string data. The other string types are owned and borrowed variants that can store
    // text in different encodings or be represented in memory differently.AsMut

    // CREATING A NEW STRING
    let mut s = String::new();
    // this creates a new empty string s, which we can load data into. If we have data to
    // load initially, we can use the to_string method (available to any type that implements
    // the Display trait)
    let data = "initial contents";
    let s = data.to_string();
    // the method also works on a string literal directly:String
    let s = "initial contents".to_string();
    // we can also use the function String::from. this is equivalent to to_string
    let s = String::from("initial contents");

    // Strings are UTF-8 encoded, so we can include any properly encoded data in them:
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // UPDATING A STRING
    // String can grow in size and contents can change, similar to vectors. you can use the (+)
    // operator or format! macro to concatenate String values
    // APPENDING A String WITH push_str AND push
    // push_str can be used to grow a string by appending a string slice
    let mut s = String::from("foo");
    s.push_str("bar");
    // now, s=foobar
    // push_str takes a string slice bc we don't necessarily want to take ownership of the parameter
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    // it would be unfortunate if we weren't able to use s2 after appending its contents to s1
    // if push_str too ownership of s2, we wouldn't be able to print it's value at the end

    // the push method takes a single character as a parameter and adds it to the String
    let mut s = String::from("lo");
    s.push('l');

    // Concatenation with the + operator or the format! macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // not s1 has been moved here anc can no longer be used
    // the string s3 will contain Hello, world!. The reason s1 is no longer valid after the addition
    // and the reason we used a reference to s2 has to do with the signature of the method that gets
    // called when we use the + operator. The + operator uses the add method, whos signature looks
    // something like this:
    // fn add(self, s: &str) -> String {}
    // this is weird. Essentially, + requires a String (i.e. s1) be fully passed in as 'self'. The
    // function takes ownership, appends a reference to a string slice (&s2), then passes ownership back
    // after this, s1 is unusable, s2 and s3 are fine. This is more efficient than copying (there's also
    // some deref coercion used to get String into a string slice)

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    // concatenation with + can get unwieldy

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    // the fromat! macro can make short work of this. This is formatted similar to println!. Additionally,
    // it doesn't take ownership of any of the parameters, so s1 is still intact.

    // INDEXING INTO STRINGS
    // in other programming languages, you can access individual characters of a string via an index. Not
    // so in Rust; you'll get an error. Rust strings don't support indexing.

    // Internal Representation
    // String is a wrapper over Vec<u8>
    let len = String::from("Hola").len();
    // in this case, len=4, meaning the vector storing the string "Hola" is 4 bytes long
    let len = String::from("Здравствуйте").len();
    // in this case the answer is 24 (not 12) because each Unicode scalar takes 2 bytes of
    // storage instead of 1. Therefore, an index of the string's bytes will not always
    // correlate to a valid Unicode scalar value
    // example:
    // let hello = "Здравствуйте";
    // let answer = &hello[0];
    // What should answer be? when encoded in UTF-8, the first byte of З(Ze) is 208 and the second is 151, so 
    // answer should be 208, but 208 is not a valid character on its own, nor would it actually want to be returned
    // by the user. To avoid this, rust won't compile the code at all

    // Bytes and Scalar Values and Grapheme Cluster! Oh My!
    // another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust's perspective:
    // as bytes, scalar values, and grapheme clusters (the closest to 'letters')
    // looking at the Hini word नमस्ते written in Devanagari script, it is stored as a vector of u8 values that look like: 
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // That is 18 bytes and is how computers ultimately store the data. 
    // We can look at them as Unicode scalar values (which are what Rust's char type is)
    // ['न', 'म', 'स', '्', 'त', 'े']
    // There are six char values here, but the 4th and 5th are not letters, they are diacritics that don't make sense on their
    // own.
    // Finally, we can look at them as grapheme clusters
    // ["न", "म", "स्", "ते"]
    // this is what we'd call the four letters that make up the hindi word

    // Slicing Strings
    // indexing into a string is often bad bc it's not clear what the return type of the string indexing operation should be:
    // byte, character, grapheme cluster, or string slice
    // Therefore, Rust asks you to be more specific when you need to crate a string slice
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // s will be and &str that contains the first 4 bytes of the string (Зд)
    // if the slice isn't a full character (for example &hello[0..1]), it would panic at runtime, and can crash the program

    // Methods for Iterating over Strings
    // you can perform operations over individual Unicode values with the chars method
    for c in "नमस्ते".chars() {
        println!("{}", c); // will return the 6 unicode scalar values
    }
    // we can also look at the bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b); // will return the 18 bytes
    }
    // getting grapheme clusters is complex, and isn't included in the standard library (crates at crates.io are available though)

    // Strings are not so simple
    // Essentially, developers dealing with this ahead of time makes it less likely something will break later

}