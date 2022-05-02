fn main() {
    let christmas_events = &[
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swams a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    println!("On the first day of Christmas");
    println!("My true love gave to me");
    println!("A partridge in a pear tree");

    for curr_date_num in 2..=12 {
        println!("\nOn the {}{} day of Christmas", curr_date_num, get_suffix(curr_date_num));
        println!("My true love gave to me");
        for date_num in 1..=curr_date_num {
            println!("{}", christmas_events[curr_date_num - date_num]);
        }
    }
}

fn get_suffix(num: usize) -> &'static str {
    let remainder = num % 10;
    let teen_remainder = num % 100 - remainder;

    if remainder == 2 && teen_remainder != 10 {
        return "nd";
    } else if remainder == 3 && teen_remainder != 10 {
        return "rd";
    } else {
        return "th";
    }
}