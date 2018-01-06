fn main() {
    let gifts = ["and a Partridge in a Pear Tree.",
        "Two Turtle Doves,",
        "Three French Hens,",
        "Four Calling Birds,",
        "Five Golden Rings,",
        "Six Geese a Laying,",
        "Seven Swans a Swimming,",
        "Eight Maids a Milking,",
        "Nine Ladies Dancing,",
        "Ten Lords a Leaping,",
        "Eleven Pipers Piping,",
        "Twelve Drummers Drumming,"];

    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh",
        "eigth", "ninth", "tenth", "eleventh", "twelfth"];

    for i in 0..days.len() {
        println!("On the {} day of Christmas,", days[i]);
        println!("My true love sent to me:");

        if i == 0 {
            println!("A Partridge in a Pear Tree.");
        } else {
            for gift in gifts[0 .. i+1].iter().rev() {
                println!("{}", gift);
            }
        }
        println!("");
    }
}
