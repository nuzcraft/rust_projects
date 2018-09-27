
use std::io; // bring io library into scope

fn main() {
    // this code should generate the nth fibonacci number, based on input
    // a fibonacci number is the sum of the two preceding numbers, starting with 1s

    // input, bound to empty string
    println!("This program will output the nth number of the Fibonacci Sequence. Please input a number.");
    let mut nth_number = String::new();
    // get input from the terminal
    io::stdin()
        .read_line(&mut nth_number)
        .expect("Failed to read line");
    // convert the input into a number    
    let nth_number: u32 = nth_number.trim().parse()
        .expect("Please input a number");

    // send the number to a fib function
    println!("{}", fib(nth_number));

}

fn fib(nth_number: u32) -> u32 {
    // define the preceding variables
    let mut first_preceding = 1;
    let mut second_preceding = 0;
    
    // if we ask for the first number, simply return 1
    if nth_number == 1 {
        1
    } else {
        // loop through each number, adding up the preceding numbers, then setting the
        // preceding numbers as the next ones in the list
        // because ranges exclude the final number, this is the correct range
        for _number in 1..nth_number {
            let fib_number = first_preceding + second_preceding;
            second_preceding = first_preceding;
            first_preceding = fib_number;
        };
        // return first_preceding (= fib_number) because it's within scope
        first_preceding
    }
}
