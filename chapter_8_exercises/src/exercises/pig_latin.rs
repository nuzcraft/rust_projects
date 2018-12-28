pub fn main(string: String) {
    // take a word, and turn it into pig latin

    // first, grab the first letter and see if its a vowel
    let first_letter = string.chars().next().unwrap();
    // chars returns an iterator of the characters in a reference to string
    // next pulls the next value in the iterator (the first letter)
    // unwrap returns it, provide it is Some
    let vowel: bool = match first_letter {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        _ => false,
    };

    let mut new_word = String::new();

    if vowel == true {
        new_word.push_str(&string);
        new_word.push_str("-hay");
    } else {
        let mut first_character: bool = true;
        for character in string.chars() {
            if first_character {
                first_character = false;
            } else {
                new_word.push(character);
            }
        }
        new_word.push('-');
        new_word.push(first_letter);
        new_word.push_str("ay");
    }

    println!("{}", new_word);
}