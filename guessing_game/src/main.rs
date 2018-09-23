extern crate rand; // references a dependency defined in Cargo.toml

use std::io; // bring io (input/output) library into scope. from std (standard) library
use std::cmp::Ordering;
use rand::Rng; // the Rng trait defines methods that random number generators use

fn main() {
    println!("Guess the number!"); // use macro to print to screen

    // thread_rng is an rng local to the current thread of execution and seeded by the OS
    // gen_range is defined by Rng trait. inclusive on lower bound, exclusive on upper
    // cargo doc --open will build an open documentation for included dependencies
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number); // print the secret number (used for development)
    println!("Please input your guess."); // use macro to print to screen

    loop { // this creates an infinite loop
        // declare mutable variable 'guess' and bind it to a new empty instance of String
        let mut guess = String::new();

        // io::stdin returns an instance of io::Stdin, which is a type that represents a handle
        // to the standard input for the terminal.
        // read_line takes what the user types and places it in a string (passed in as an argument)
        // the argument needs to be mutable (mut) and a reference (&). This lets us pass in a 
        // mutable reference to the guess variable we already declared
        // this will return an io::Result, which can be 'Ok' or 'Err'. expect is a method of Result
        // that will crash the program and display the message if Result is Err
        // if you don't include .expect, it will compile with a warning
        io::stdin().read_line(&mut guess) 
            .expect("Failed to read line");

        // since we want to compare numbers, we need to change guess to an integer
        // we create a new variable guess that shadows the previous one with new type
        // take the old guess variable, trim off any whitespace (like newline) and
        // parse. parse on a string turns it into a number
        // the : after guess tells Rust that we'll annotate the variable's type (u32)
        // u32 is an unsigned, 32 bit integer (good for small positive numbers)
        // similar to read_line, .expect will handle it if parse() fails (a number isn't typed)
        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!");

        // instead of crashing when the user types in a nonnumber, let them keep guessing
        // using a match lets us check for an Ok Result from the parse()
        // the _ in the Err is a catchall, we want to continue no matter how the parse() fails
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // the {} allow for us to print a variable within the string    
        println!("You guessed: {}", guess);
        
        // cmp (compare) guess, passing in a reference to secret_number. This will return
        // a variant of Ordering
        // this is an example of a match expression (similar to a case), exits when match is found
        // if Ordering::Equal is found, it will print the line, then break out of the loop
        // breaking out of the loop exits the game
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}