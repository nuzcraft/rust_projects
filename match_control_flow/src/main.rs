
// THE MATCH CONTROL FLOW OPERATOR
// match allows you to compare a value against a series of patterns and then execute code
// based on which pattern matches
// the power of match comes from the expressiveness of the patterns and the fact that the 
// compiler confirms that all possible cases are handled.

// in a match, values move down the pattern, and at the first pattern the value fits, the
// values falls into the associated code block to be usede during execution

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
// breaking down the value_in_cents function...first, list the match keyword followed by an
// expression. (similar to an if, except this can return any type, not just booleans)
// next are the match arms, each has a pattern and some code. For the first arm, the pattern
// is Coin::Penny. the => operator separates the pattern from the code. The code is just the
// value 1. A comma separates this arm from the next

// when the match executes, it compares the resulting value against the pattern of each arm
// in order. If it matches, the code is executed. If not, it moves to the next arm.

// the code associated with each arm is an expression, and the resulting value gets returned
// for the entire match expression

// curly brackets can be used if the code in an arm is expanded, such as if we wanted to
// print 'Lucky Penny' whenever a penny is valued

fn value_in_cents_lucky_penny(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// PATTERNS THAT BIND TO VALUES
// arms can bind to parts of the values that match the pattern. This is how we extract
// values out of enum variants
// for example, from 1999 to 2008, the US minted quarters with state designs, let's
// adjust our coin enum to account for this

#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum CoinWithState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// now, when we use the match expression, we can add a variable called state to the pattern
// that matches values of the variant CoinWithState::Quarter. When a quarter matches, the
// state variable will bind to the value of that quater's state, and we can use it within the
// code for that arm

fn value_in_cents_state_quarter(coin: CoinWithState) -> u32 {
    match coin {
        CoinWithState::Penny => 1,
        CoinWithState::Nickel => 5,
        CoinWithState::Dime => 10,
        CoinWithState::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// in this example, we can use the state binding of the quarter within a println! expression,
// getting the innter state value out of the Coin enum variant for Quarter

// MATCHING WITH OPTION<T>
// in the previous section(project), we wanted to the the inner T value out of the Some case
// when using Option<T>; we can handle option T using match! Instead of comparing coins, we
// we can compare the variants of Option<T>

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// this code is specifically designed to add one to a value that may or may not exist. If it
// is none, none is returned, else, 1 is added

// MATCHES ARE EXHAUSTIVE
// we must exhaust every last possibility in order for the code to be valid and compile. This
// is especially true in the case of Option<T> where we much explicitly handle None values

fn main() {
    
    value_in_cents(Coin::Dime);
    value_in_cents_lucky_penny(Coin::Penny);
    value_in_cents_state_quarter(CoinWithState::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // THE _ PLACEHOLDER
    // Rust has a pattern we can use when we don't want to list all possible values.
    // For example, a u8 can have valid values 0 through 255. If we only care about a few of
    // them, we don't want to have to list them all

    let some_u8_value = 7u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => ()
    }

    // the _ pattern will match any value, by putting it after our other arms, it will match
    // any case that wasn't specified before it. The () is just the unit value, so nothing
    // will happen in the _ case.

    // the match expression can be a bit wordy in a situation in which we only care about one
    // of the cases. For this situation, Rust provides if let.

    // CONCISE CONTROL FLOW WITH IF LET

    // if let syntax lets you combind if and let into a less verbose way to handles values
    // that match one pattern while ignoring the rest. Below is a match example:
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // if let provides a shorter way of doing this
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // the syntax takes a pattern and expression separated by an =. It works the same as a 
    // match, where the expression is given to the match  and the pattern is its first arm
    // However, this loses the exhaustive checking that match inforces

    // we can also include an else clause that can handle the same block of code that the _
    // case of a match would handle. See below for the same example, one with match and the 
    // other with if let

    let mut count = 0;
    let coin = CoinWithState::Quarter(UsState::Alabama);
    match coin {
        CoinWithState::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }

    let coin = CoinWithState::Quarter(UsState::Alaska);
    if let CoinWithState::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }

}
