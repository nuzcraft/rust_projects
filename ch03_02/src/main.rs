fn main() {
    // this chapter is on data types
	
	// Rust is statically typed; it must know all data types at compile time
	// parse() converts a string to a number; u32 defines guess as a 32 bit integer (consistent
	// witht the results of parse)
	let guess: u32 = "42".parse().expect("Not a number!");
	println!("guess: {}", guess);
	// if we forget to add thte type annotation, we'll get an error
	// let guess = "42".parse().expect("Not a number!");

	//SCALAR TYPES
	// Rust has 4 primary scalar types: integers, floating-point numbers
	// booleans and characters
	// Integers can be signed (+ or -) or unsigned (assumed positive)
	// i8, i16, i32, i64, i128, isize - signed
	// u8, u16, u32, u64, u128, usize - unsigned
	// Rust default is i32, and is generally the fastest, even on 64 bit machines

	// Floating point numbers have decimal points: 
	// f32 (single-precision float)  or f64 (default)(double-precision float)
	let x = 2.0; // f64
	println!("x: {}", x);
	let y: f32 = 3.0; // f32
	println!("y: {}", y);

	// addition
	let sum = 5 + 10;
	println!("sum: {}", sum);
	// subtraction 
	let difference = 95.5 - 4.3;
	println!("difference: {}", difference);
	// multiplication
	let product = 4 * 30;
	println!("product: {}", product);
	// division
	let quotient = 56.7 / 32.2;
	println!("quotient: {}", quotient);
	// remainder
	let remainder = 43 % 5;
	println!("remainder: {}", remainder);

	// booleans can be true or false and have a type bool; booleans are one byte in size
	let t = true;
	println!("t: {}", t);
	let f: bool = false; // with explicit type annotation
	println!("f: {}", f);

	// char type represents a Unicode Scalar Value; specified with single quotes as
	// opposed to a string literal, which uses double
	let c = 'z';
	println!("c: {}", c);
	let z = 'ℤ';
	println!("z: {}", z);
	let heart_eyed_cat = '😻';
	println!("heart_eyed_cat: {}", heart_eyed_cat);

	// COMPOUND TYPES
	// 2 primitive types, tuples and arrays; used to group multiple values into one type
	// tuples is a general way of grouping values
	// they have a fixed length and cannot grow or shring
	let tup: (i32, f64, u8) = (500, 6.1, 1); // tup is bound to the entire tuple
	// we can use pattern matching to pull the individual values out of the tuple
	let (x, y, z) = tup; // this is called destructuring
	println!("x: {}", x);
	println!("y: {}", y);
	println!("z: {}", z);
	// another way to do it is to use a . followed by the index that we want
	let five_hundred = tup.0;
	println!("five_hundred: {}", five_hundred);
	let six_point_one = tup.1;
	println!("six_point_one: {}", six_point_one);
	let one = tup.2;
	println!("one: {}", one);

	// arrays are similar; every element must have the same type, but are still a fixed length
	let a = [1, 2, 3, 4, 5];
	// arrarys allocate data on the stack rather than the heap
	// the vector type is similar to an array, but can grow and shrink
	// arrays are useful when you know you'll never need to add or remove items
	// e.g. months in a year
	let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
	// arrays are denoted by [type; number]
	let a: [i32; 5] = [1, 2, 3, 4, 5];
	// you can access the elements of an array with an index
	let first = a[0];
	println!("first: {}", first);
	let second = a[1];
	println!("second: {}", second);
	// trying to access an element pas the end of the array will result compile
	// but will produce a runtime error
	let index = 10;
	let element = a[index]; //Rust will panic here, noticing that the index is greater
	// than the length of the array and will exit the program immediately
	println!("The value of element is: {}", element); // this line is never run
}
