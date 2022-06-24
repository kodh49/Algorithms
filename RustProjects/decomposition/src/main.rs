use std::io;

fn main() {
    let input = scanln();
    let mut result = 0;
    for iterator in 0..input {
        let construct = constructor(iterator);
        if input == construct {
            result = iterator;
            break;
        }
    }
    println!("{}", result);
}

fn constructor(num:u32) -> u32 {
    let mut number = num;
    let mut sum = num;
    while number/10 != 0 {
        sum += number % 10;
        number /= 10;
    }
    sum += number;
    sum
}

fn scanln() -> u32 {
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Cannot read line");
    let string: u32 = match string.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    string
}