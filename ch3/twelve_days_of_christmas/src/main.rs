fn main() {
    let words_one_through_twelve = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let twelve_days_of_christmas = [
        "a partridge in a pear tree.",
        "two turtle doves,",
        "three French hens,",
        "four calling birds,",
        "five golden rings!",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a milking,",
        "nine Ladies dancing,",
        "ten Lords a leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming,",
    ];

    // This iterates over the array words_one_through_twelve and prints on the first day of Christmas the verse for each word
    for (i, word) in words_one_through_twelve.iter().enumerate() {
        println!("On the {} day of Christmas my true love gave to me", word);

        //This is a nested loop that takes the current value of i + 1 and iterates backwards through the array twelve_days_of_christmas until it reaches 0
        for j in (0..i + 1).rev() {
            // This makes sure that the first time the first verse is printed, it doesn't print "and" and only prints the and on the last index.
            if j == 0 && i != 0 {
                println!("and {}", twelve_days_of_christmas[j]);
            } else {
                println!("{}", twelve_days_of_christmas[j]);
            }
        }
    }
}
