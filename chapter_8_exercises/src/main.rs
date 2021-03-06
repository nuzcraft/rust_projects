use std::io;

// each exercise lives in the exercise module (which is a directory)
mod exercises;

fn main() {
    let mut exit: bool = false;
    while exit == false {
        println!("Please choose an exercise:");
        println!("Type 'exit' at any time to exit.");
        println!("a: Mean, Median Mode");
        println!("b: Pig Latin");
        println!("c: Text Interface");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        // each exercise is it's own module within the exercise module. Each has 
        // a 'main' function used to do its thing
        match choice.as_str().trim() { // trim is used to pull off the newline
            "a" => {
                exercises::mean_median_mode::main();
                println!(""); // print a newline
                },
            "b" => {
                exercises::pig_latin::main();
                println!(""); // print a newline
                },
            "exit" => {
                println!("Goodbye!");
                exit = true;
            }
            _ => (),
        }
    }
}
