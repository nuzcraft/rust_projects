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
}
