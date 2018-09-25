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
}
