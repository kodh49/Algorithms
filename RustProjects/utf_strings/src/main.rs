fn main() {
    let num = 100;
    // var.to_string() -> Convert existing stack-only data(that has display trait) into String
    let mut num = num.to_string();

    // String.push_str("blah blah blah") -> Add string slice to existing String var
    num.push_str(" + 10 = 110"); // Parameter can only be a string slice (&str)
    num.push('!'); // Parameter can only be char (single character enclosed with single quote)
    println!("{}", num);

    // Concatenation of String can be achieved by two different ways
    let str1 = String::from("Compiling");
    let str2 = String::from("Running");
    let str3 = String::from("Completed");
    // First way of doing this: use "+" operator between two string variables
    let str4 = str1+" → "+&str2+" → "+&str3; // In the case of "+" operator, every string expect for the first one should be referenced(&)
    println!("Concatenated using \"+\" operator: {}", &str4);
    // Second way of doing this: format! Macro: This is useful when you're concatenating more than 3 strings together, or adding special rules into them
    let str5 = format!("{} → {}", str2, str3);
    println!("Concatenated using \"format!\" function: {}", &str5);

    // Indexing through each character that consists of entire string
    let kor = String::from("안녕하세요?");
    let kor_slice = &kor[6..9]; // 한국어는 한 글자당 3 Bytes -> Need to index through an array
    println!("Sliced Korean String:{}", kor_slice);

    // Iterating over Strings -> Indexing in different way
    let utf_str = "こんにちは";
    // Print each character in char-type
    for utf_char in utf_str.chars() {
        print!("{}\n", utf_char);
    }
    // Print each character in its own byte value
    for utf_byte in utf_str.bytes() {
        print!("{}\n", utf_byte);
    }
}