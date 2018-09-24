fn main() {
    println!("Hello, world!");

    // to call another function, call it by its name with two parenthesis
    another_function();
    // if the function takes a parameter, you can pass it into the parenthesis
    another_function_param(5);
    // more parameters are separated by a comma
    another_function_two_params(5, 6);
}

// functions start with fn. the curly brackets tell the compiler where it begins and ends
// note this is defined after the main function. functions can be defined anywhere
fn another_function() {
    println!("Another function.");
}

// you can define a function to take parameters. parameter types must be declared
fn another_function_param(x: i32) {
    println!("The value of x is: {}", x);
}

// additional parameters can be separated by a comma
fn another_function_two_params(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
