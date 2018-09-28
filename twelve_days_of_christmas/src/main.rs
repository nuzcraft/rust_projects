fn main() {
    // the program will print the lyrics to 12 days of chrismas

    // first get the words for all the days put in an array
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelvth",
    ];

    // then get all the different gifts into an array
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four colly birds",
        "FIVE GOLDEN RINGS",
        "Six gees a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    // days.len() returns the number of items in the days array
    // 0.. is the start of a range, and ranges don't include
    // the final number, thus 0..12 doesn't include the number 12
    // AKA 0-11 (12 numbers)
    for number in 0..days.len() {
        // print the first line of each stanza, using the correct day
        // we pass in the index of the day we want (0 for first),
        // (11 for twelvth)
        println!(
            "On the {} day of Christmas my true love gave to me",
            days[number]
        );
        // we add another loop because we want to repeat all the gifts
        // for each stanza, but this time in reverse order (hence rev())
        // additionally, we want to add "and" after two turtle doves
        // which we do by starting with a blank string, then changing it 
        // when we hit the correct row
        for new_number in (0..number + 1).rev() {
            let mut and = "";
            if new_number == 1 {
                and = " and";
            }
            println!("{}{}", gifts[new_number], and);
        }
    }
}
