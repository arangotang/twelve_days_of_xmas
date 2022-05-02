fn main() {
    let christmas_events = &[
        "And a song for the Christmas tree",
        "Two candy canes",
        "Three boughs of holly",
        "Four colored lights",
        "A shining star",
        "Little silver bells",
        "Candles a-glowing",
        "Gold and silver tinsel",
        "A guardian angel",
        "Some mistletoe",
        "Gifts for one and all",
        "All their good wishes",
    ];

    println!("On the first day of Christmas");
    println!("My good friends brought to me");
    println!("A song and a Christmas tree");

    for curr_date_num in 2..=12 {
        println!("\nOn the {}{} day of Christmas", curr_date_num, get_suffix(curr_date_num));
        println!("My good friends brought to me");
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