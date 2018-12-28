use std::collections::HashMap;
pub fn main() {
    // Given a list of integers, use a vector and return the mean, median, and mode of the list
    let mut v: Vec<i32> = vec![1, 5, 31, 0, 59, 2, 0, 29, 29]; 
    v.sort(); // sort the vector (must be mutable)
    
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
        let first_median_index: usize = (len as usize) / 2;
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
    println!("The mode(s) are: {:?}", mode_v);
}

// this function is a simple match on the remainder 
// note, it is not (pub) so it can only be referenced from inside this file
fn is_odd(integer: i32) -> bool {
    match integer % 2 {
        0 => false,
        _ => true,
    }
}