fn main() {
    // this will fail because the first assignment of x designates it
    // as immutable
    // let x = 5;

    // if we make the first x mutable instead, it will compile
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);    

    // an immutable variable is different from a constant. constants 
    // are declare with const, not let, and the type MUST be annontated
    // constants must be set to a constant expression, not the result
    // of a function that could change at runtime.
    // best practices make constants all uppercase
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // this is an example of shadowing. using the let keyword when
    // lets us effectively replace the variable with a new one of the
    // same name (which can still be immutable). declaration of the
    // new one can reference the old one, or change the type. 
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // this is an example of shadowing to change the type
    let spaces = "   ";
    println!("spaces: ~{}~", spaces);
    let spaces = spaces.len();
    println!("spaces: ~{}~", spaces);

    // mutable variables cannot have their types changed, 
    // this code will not compile
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
