use std::collections::HashMap;
use std::io;

pub fn main() {

    println!("Welcome to Mean, Median, Mode.");
    // create input loop that waits for a valid input

    let mut integer_input: bool = false;
    let mut int: i32 = 0; // start with 0
    let mut exit: bool = false;

    while integer_input == false { // we only want to continue if an integer is input
        // ask the user to print an integer
        println!("Please input an integer.");
        // take input from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        // if 'exit' is typed, exit set the exit flag to true
        match input.as_str().trim() {
            "exit" => {
                exit = true;
                integer_input = true;
            },
            _ => (),
        };
        
        if exit == false {
            // try to parse the input into an integer
            match input.trim().parse() {
                Ok(num) => { // if good, set the int to the number, and set the flag to true
                    int = num;
                    integer_input = true;
                },
                Err(_) => { // if error, display an error message, and let them try again
                    println!("You typed: {}. Please type an integer.", input.trim());
                    continue;
                },
            }
        }
    }
    
    if exit == false {
        // create a blank vector
        let mut v: Vec<i32> = Vec::new();
        // push the typed integer into the vector
        v.push(int);
        println!("The current vector is: {:?}", v);

        // create a loop to let the user add integers if desired
        loop {
            // ask for an integer, place it in a new string each time
            println!("Please input an integer or 'end'.");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2)
                .expect("Failed to read line");
            match input2.as_str().trim() {
                "end" => break, // if 'end' is typed, break our of the loop
                "exit" => {
                    exit = true;
                    break;
                },
                _ => {
                    match input2.trim().parse() {
                        Ok(num) => { // if good, set the int to the number
                            int = num;
                        },
                        Err(_) => { // if error, display an error message, and let them try again
                            println!("You typed: {}. Please type an integer.", input2.trim());
                            continue;
                        },
                    }
                    v.push(int);
                    println!("The current vector is: {:?}", v);
                },
            }
        }

        if exit == false {
            v.sort(); // sort the vector (must be mutable)
            println!("The vector is {:?}", v);
            
            // Get the number of elements in the vector; assert that this cannot be 0
            let len: i32 = v.len() as i32;
            assert_ne!(v.len(), 0);

            // get and print the mean by looping through the vector
            // at the same time, look for the median
            let mut sum: i32 = 0;
            for i in &v {
                sum += i;
            }
            // cast the sum and len as floats for decimal division
            let mean: f64 = sum as f64 / len as f64;
            println!("The mean is: {}", mean);

            // find if there's an even or odd number of of items in the list
            if is_odd(len) {
                // if theres an odd number, we can find the middle one and print it
                let median_index: usize = (len as usize - 1) / 2;
                println!("The median is: {}", v[median_index]);
            } else {
                // if there's an even number, we need to find the average of the two middle values
                let first_median_index: usize = ((len as usize) / 2) - 1;
                let second_median_index = &first_median_index + 1;
                let median: f64 = (v[first_median_index] as f64 + v[second_median_index] as f64) / 2 as f64;
                println!("The median is: {}", median);
            }

            // create a HashMap for each entry in the vector, counting the number
            // of times each number is used. this code is pulled from chapter 8 (and is really cool)
            let mut mode_map = HashMap::new();
            for i in &v {
                let count = mode_map.entry(i).or_insert(0);
                * count += 1;
            }
            // loop through the hash map, finding the highest entries, then, create a
            // new vector with those values
            let mut highest_value: i32 = 0;
            for (_key, value) in &mode_map {
                if value > &highest_value {
                    highest_value = *value;
                }
            }
            let mut mode_v: Vec<i32> = Vec::new();
            for (key, value) in &mode_map {
                if value == &highest_value {
                    mode_v.push(**key);
                }
            }
            // IMPORTANT: {:?} is the debug mode print, vectors do not have a legitimate print mode
            mode_v.sort();
            println!("The mode(s) are: {:?}", mode_v);
        } else {
            println!("Exiting Mean, Median, Mode. Goodbye.", )
        }
    } else {
    println!("Exiting Mean, Median, Mode. Goodbye.");
    }
}

// this function is a simple match on the remainder 
// note, it is not (pub) so it can only be referenced from inside this file
fn is_odd(integer: i32) -> bool {
    match integer % 2 {
        0 => false,
        _ => true,
    }
}