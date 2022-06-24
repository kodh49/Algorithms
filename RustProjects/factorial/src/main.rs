use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Cannot read line");
    let n:u64 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    println!("{}", factorial(n));
}

fn factorial(n:u64) -> u64{
    if n<=1 {
        1
    }
    else {
        factorial(n-1)*n
    }
}