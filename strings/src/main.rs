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
}