use std::io;

pub fn main() {

    let mut exit: bool = false;

    println!("Welcome to Pig Latin!");
    println!("Please type a word and we'll turn it into pig latin.");
    // take a word, and turn it into pig latin
    let mut string = String::new();
    io::stdin().read_line(&mut string)
        .expect("Failed to read line");

    // if 'exit' is typed, exit set the exit flag to true
    match string.as_str().trim() {
        "exit" => {
            exit = true;
        },
        _ => (),
    };

    if exit == false {
        // first, grab the first letter and see if its a vowel
        let first_letter = string.chars().next().unwrap();
        // chars returns an iterator of the characters in a reference to string
        // next pulls the next value in the iterator (the first letter)
        // unwrap returns it, provide it is Some

        // we aren't worried about numbers or characters here, let them put in what they wish

        // match the first letter on a static list of values, return true or false
        let vowel: bool = match first_letter {
            'a' => true,
            'e' => true,
            'i' => true,
            'o' => true,
            'u' => true,
            _ => false,
        };

        // create an empty String to hold our new word
        let mut new_word = String::new();

        // if it's a vowel, add -hay to the end
        if vowel == true {
            new_word.push_str(&string.as_str().trim());
            new_word.push_str("-hay");
        } else {
            // not a vowel, gets complicated
            let mut first_character: bool = true;
            for character in string.as_str().trim().chars() {
                if first_character {
                    first_character = false; // if it's the first character, skip it
                } else {
                    new_word.push(character); // push the rest of the letters into the new_word
                }
            }
            // now add -(first letter)ay to finish off the pig latin
            new_word.push('-');
            new_word.push(first_letter);
            new_word.push_str("ay");
        }

        // print your new word
        println!("Your word is: {}", new_word);
    } else {
        println!("Exiting Pig Latin. Goodbye.");
    }
}