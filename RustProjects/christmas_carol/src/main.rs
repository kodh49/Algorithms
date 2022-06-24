fn main() {
    let verse = ["A partridge in a pear tree", "Two turtle doves", "Three French hens", "Four calling birds", "Five gold rings", "Six geese a-laying",
    "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];
    let day = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth", "tenth", "eleventh", "twelveth"];
    for index in 0..day.len() {
        println!("On the {} day of Christmas my true love sent to me", day[index]);
        for itr in (0..index+1).rev() {
            println!("{}", verse[itr]);
        }
    }
}
