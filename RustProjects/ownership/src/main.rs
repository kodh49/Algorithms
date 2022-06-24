fn main() {
    let mut some_string = String::from("I'm so glad to meet you");
    some_string.clear();
    let word = first_word(&some_string);
    println!("First word: {}", word);
}

fn first_word(string: &String) -> &str {
    // Change the String into an array of bytes
    let bytes = string.as_bytes();
    // Iterate through each element of an array
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // b'x' makes the compiler understand the character x in bytes
            return &string[..i];
        }
    }
    &string[..]
}