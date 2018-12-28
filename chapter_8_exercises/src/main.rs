use std::io;

// each exercise lives in the exercise module (which is a directory)
mod exercises;

fn main() {

    println!("Please choose an exercise:");
    println!("a: Mean, Median Mode");
    println!("b: Pig Latin");
    println!("c: Text Interface");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
        .expect("Failed to read line");

    // each exercise is it's own module within the exercise module. Each has 
    // a 'main' function used to do its thing
    match choice.as_str().trim() { // trim is used to pull off the newline
        "a" => {exercises::mean_median_mode::main();},
        "b" => {exercises::pig_latin::main(String::from("hello"));},
        _ => println!("Please choose one of the given options."),
    }
}
