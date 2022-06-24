use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input:i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let result = fibonacci(input);
    println!("{}", result);
}

fn fibonacci(n:i32) -> u64 {
    if n > 2 {
        return fibonacci(n-1) + fibonacci(n-2);
    }
    else if n >= 1 {
        return 1;
    }
    else {
        return 0;
    }
}