use std::io;

fn main() {
    let test_cases = get_value();
    for _ in 0..test_cases {
        // Input for floor & room number
        let k = get_value(); // k = floor
        let n = get_value(); // n = room
        let residents = residency(k, n); // Start calculation
        println!("{} Residents", residents);

    }
}

fn get_value() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Cannot read from line");
    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    input
}

fn residency(k: i32, n: i32) -> i32 {
    if k == 0 { // floor = 0
        n
    }
    else {
        let mut sum = 0;
        for x in 1..n+1 {
            sum += residency(k-1, x);
        }
        sum
    }
}