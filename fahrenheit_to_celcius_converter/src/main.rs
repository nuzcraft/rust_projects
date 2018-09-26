fn main() {
    // the purpose of this program is to convert Fahrenheit to Celcius
    // or vice versa

    // first, we need inputs: temperature and units
    // there was some confusion with units; converted everything to a float
    let temp: f64 = 68 as f64;
    let unit = "F"; // must be a C or F

    // if I define these up here, they get flagged as 'unused'
    // let _result = "unknown"; // the result is unknown, will be set later
    // let _result_unit = "unknown"; // the result unit is also unknown

    // check the unit, and send it to the correct conversion function
    if unit == "F" { // converting F to C
        let result = f_to_c(temp);
        let result_unit = "C";
        // slight duplication of code,  but I don't want it to run if the unit is wrong
        println!("{}{} => {}{}", temp, unit, result, result_unit);
    } else if unit == "C" { // converting C to F
        let result = c_to_f(temp);
        let result_unit = "F";
        println!("{}{} => {}{}", temp, unit, result, result_unit);
    } else {
        // wrong unit entered
        println!("Please enter C or F for unit.");
    };
}

fn f_to_c(temp: f64) -> f64 {
    // convert fahrenheit to celcius
    // can't do math functions when units are different (even integer and floats)
    (temp - 32 as f64) / 1.8
}

fn c_to_f(temp: f64) -> f64 {
    // convert celcius to fahrenheit
    temp * 1.8 + 32 as f64
}
