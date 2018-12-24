#![allow(unused_variables)]
fn main() {
    println!("Hello, world!");
    // STORING KEYS WITH ASSOCIATED VALUES IN HASH MAPS
    // the type HashMap<K, V> stores a mapping of keys of type K to values of type V.It does this with a 
    // hashing function, which determines how it places these keys an values into memory. Many programming
    // languages support this kind of data structure, but they often use a different name, such as hash, map
    //, object, hash table, dictionary, or associative array.

    // CREATING A NEW HASH MAP
    // you can create an empty hash map with new and add elements with insert
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // this hash map has keys of type String and values of type i32. All keys must have the same type
    // and all values must have the same type

    // another way of constructing a hash map is by using the collect method on a vector of tuples
    // the collect method gathers data into a number of collection types, including HashMap. 
    // in the following example, we have two separate vectors: one of team names, and another of scores
    // we use the zip method to create a vector of tuples, where "Blue" is paired with 10 etc. Then
    // use the collect method to turn that vector of tuples into a has map
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // the type annotation HashMap<_, _> isneeded because it's possible to collect into many different
    // data structures and Rush doesn't know which you want unless you specify. We use the underscores
    // becuase Rust can infer the types that the hash map contais based on the types of data in the vectors

    // HASH MAPS AND OWNERSHIP
    // For types that implement the Copy trait, like i32, the values are copied into the Hash Map. For owned values
    // like String, the values will be moved and the hash map will be the owner of those values
    let field_name = String::from("Favorite Color");
    let field_value =  String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // they've been moved into the hash map with the call to insert
    // we can insert references into the hash_map, then the values must be valid for at least as long as the 
    // hash map is valid

    // ACCESSING VALUES IN A HASH MAP
    // we can use get to get a value from a hash map
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    // Here, score will have the value associated with the Blue team, and the result will be
    // Some(&10). get returns an Option<&V>; if there's no value for the key, get will return None.
    // take care, and make sure you handle both optionss

    // We can iterate over each key/value pair in a hash map as we do with vectors with a for loop
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // the code prints each pair in an arbitrary order

    // UPDATING A HASH MAP
    // each key can only have one value associated with it at any time. when you wanna change it, you 
    // need to decide how to handle the case the key already has a new value.
    
    // overwriting a value
    // if we insert a key and value then insert the same key witha different value, the value will be replaced
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
    // this will print {"Blue": 25}, as 10 was overwritten

    // Only inserting a value if the key has no value
    // It's common to check whether a particular key has a value and, if it doesn't, insert a value for it. 
    // Hash Maps have a special API for this called entry that takes the key as a parameter. The return value 
    // of the entry method in an enum called Entry that represents a value that might or might not exist
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    // the or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key
    // if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference
    // to the new value. 
    // the first entry key will check for Yellow, find nothing, and insert 50
    // the second entry will check for Blue, find 10, and not change the hash map

    // Updating a Value Based on the Old Value
    // lets keep track of how may times we insert a word into a hash map. If it's the first time, we'll insert a 0
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    // the or_insert method actual returns a mutable reference (&mut V) to the value for this key. Here, we store that mutable
    // reference in the count variable, so in order to assign to tha value, we must first dereference count using the asterisk.
    // The mutable reference goes out of scope at the end of the for loop, so all these changes are safe and allowed by the
    // borrowing rules

    // Hashing Functions
    // the default HashMap uses a "cryptographically strong", hashing function. It is not the fastest. If you need faster
    // you can specify it. Additional hashing functions can be found on crates.io.

    // CH 8 Summary
    // Vectors, strings, and hash maps provide a large amount of functionality necessary in programs when you need to store,
    // access, and modify data. here are some excercises to do
    // * Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in 
    // the middle position), and mode (the value that occurs the most often; a hash map will be helpful here) of the list.
    // * convert strings to pig latin. The first consonant of each word is moved to the end of the word and "ay" is added, 
    // so first becomes irst-fay. Words that start with a vowel have "hay" added to the end instead (apple-hay). Keep in mind
    // details about UTF encoding
    // * Useing a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
    // for example, "Add Sally to Engineering", or "Add Amir to Sales." Then let the user retrieve a list of all people in a department 
    // or all people in the company by department sorted alphabetically

}
