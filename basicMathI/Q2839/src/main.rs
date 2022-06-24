use std::io;

fn main() {
    let mut N = String::new();
    io::stdin().read_line(&mut N).expect("unable to read line");
    let N: i32 = match N.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    println!("{}", delivery(N));
}

fn delivery(N: i32) -> i32 {
    if N/5 > 0 {
        let mut num_5: i32 = N/5;
        let mut num_3: i32 = (N%5)/3;
        let mut remainder = (N%5)%3;
        while remainder != 0 {
            if num_5 > 0 {
                num_5 -= 1;
            } else {
                return -1;
            }
            num_3 = (N-num_5*5)/3;
            remainder = (N-num_5*5)-num_3*3;
        }
        num_5+num_3 // Total number of packages
    } else {
        if N == 3 {
            1
        }
        else {
            -1
        }
    }
}
