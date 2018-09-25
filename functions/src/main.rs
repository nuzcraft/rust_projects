fn main() {
    println!("Hello, world!");

    // to call another function, call it by its name with two parenthesis
    another_function();
    // if the function takes a parameter, you can pass it into the parenthesis
    another_function_param(5);
    // more parameters are separated by a comma
    another_function_two_params(5, 6);

    // function bodies may end in an expression (the functions above do not)
    // Rust is an expression-based language, so these functions differ significantly
    // Statements are instructions that do not return a value
    // Expressions evaluate to a resulting value

    // this is a statement; a function can also be a statement
    // if it doesn't return anything
    let y = 6;
    // you can't assign a let statement to another variable, you'll get an error
    // let x = (let y = 6); // let y = 6 doesn't return a value, so x can't be assigned
    // this is different from other languages; x = y = 6 doesn't work in Rust

    // Expressions evaluate to somthing, are make up most everything else in Rust
    // calling a function, calling a macro, the block used to create new scopes {},
    // are all expressions
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
    // the expression {
    //     let x = 3;
    //     x + 1
    // };
    // evaluates to 4. The final line DOES NOT end in a semicolon, as expressions
    // don't include semicolons. A semicolon indicates a statement instead

    // the following function returns something (is an expression)
    // and the result can be assigned to a variable
    let x = five();
    // because five() returns 5, this line is the same as the following
    let x = 5;

    // the following function takes a parameter AND returns a value
    let x = plus_one(5);
    println!("The value of x is: {}", x);
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

// functions can return values; the return type must be declared with ->
fn five() -> i32 {
    5
}

// both parameters and return values need to have their types defined
fn plus_one(x: i32) -> i32 {
    x + 1 // this doesn't end in a semicolon, bc this is an expression (return value)
          // x + 1; would cause an error because it isn't returning the type specified
}
