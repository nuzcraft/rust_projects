// each exercise lives in the exercise module (which is a directory)
mod exercises;

fn main() {
    // each exercise is it's own module within the exercise module. Each has 
    // a 'main' function used to do its thing
    exercises::mean_median_mode::main();

    exercises::pig_latin::main(String::from("hello"));
}
