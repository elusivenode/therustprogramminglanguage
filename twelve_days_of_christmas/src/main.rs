fn main() {
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let lyrics: [&str; 12] = [
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "12 drummers drumming",
    ];
    let mut index: usize = 0;
    while index < days.len() {
        println!(
            "On the {} day of Christmas\nMy true love gave to me",
            days[index]
        );
        if days[index] == "first" {
            println!("A partridge in a pear tree");
        } else {
            for number in (0..index + 1).rev() {
                println!("{}", lyrics[number as usize]);
            }
        }
        println!("\n");
        index = index + 1
    }
}
