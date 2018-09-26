fn main() {
    // the most common constructs that let you control the flow of execution of Rust 
    // are if expressions and loops
    // if expressions
    // if this condition is met, run this code, if this condition is not met, do not
    // let number = 3;
    let number = 7;
    if number < 5 {
        // if the number is 3, this will run
        println!("condition was true");
    } else {
        // if the number is 7, this will run
        println!("condition was false");
    }
    // importantnly, the condition must be a bool
    // if number {
    //     println!("number was three");
    // }
    // will fail to compile
    if number != 0 {
        println!("number was something other than zero");
    }

    // else if can be used to handle multiple conditions. when running multiple
    // conditions, it will return the first that is true
    // often, else if's can be refactored into a match
    let number = 7;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // because if is an expression, it can be used in a let statement
    // in an if, all arms must evaluate to the same type (e.g. both integers)
    let condition = true;
    let number = if condition {
        5
    } else {
        6
        //"six" // if one arm is 5 and the other is "six", it will throw an error
    };
    println!("The value of number is: {}", number);

    // rust has 3 types of loops: loop, while, for
    // loop is for executing the same block of code over an over until
    // you explicitly tell Rust to stop
    loop {
        println!("again!");
        break // use the break keyword to exit a loop
    }

    // adding a result to the break expression will return it frm the broken loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20); // this macro will cause a panic if the numbers are not equal

    // while to continue looping while a condition is true
    // for the following loop, we loop, subtracting 1 from number until it hits zero
    // once it hits zero, the loop breaks and we continue the code
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    // you can use a while loop to loop over the elements of a collection (like an array)
    // this code will return each element of the array in turn
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1
    }
    // this approach is error prone, as the index length must be correct
    // this is slower as well, the compiler adds runtime code to check every element
    // on every iteration through the loop

    // a for loop is more concise
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    // notice how we didn't need to define the length of the array ahead of time
    // for loops are the most commonly used loop construct in Rust. Even the while
    // loop example above (321LIFTOFF) can be adjusted to use a for loop and a range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    
}
