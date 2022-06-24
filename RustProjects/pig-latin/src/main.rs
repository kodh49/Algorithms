use std::io::stdin;

fn main() {
    let data = read_data();
    let mut first_char = b' ';
    let mut new_str: Vec<u8> = Vec::new();
    for (i, &character) in data.as_bytes().iter().enumerate() {
        if i == 0 {
            // match whether the first character is a vowel or a consonant
            match character {
                b'a' => continue,
                b'e' => continue,
                b'i' => continue,
                b'o' => continue,
                b'u' => continue,
                _ => first_char = character,
            }
        }
        else {
            new_str.push(character);
        }
    }
    new_str.pop(); // Remove the spacing character (\n) from the vector
    new_str.push(b'-');
    match first_char {
        b' ' => {
            new_str.push(b'h');
            new_str.push(b'a');
            new_str.push(b'y');
        },
        _ => {
            new_str.push(first_char);
            new_str.push(b'a');
            new_str.push(b'y');
        },
    }
    // println!("New String {:?}", new_str);
    // Iterator might add extra \n to the end blc above print expression includes ASCII 10 (\n) as default
    let final_str = match String::from_utf8(new_str) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 Sequence: {}", e),
    };
    println!("{}", final_str);
}

fn read_data() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("Cannot read line");
    line
}

/* How this program should work
Convert strings to pig latin.
The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
Keep in mind the details about UTF-8 encoding!
*/