use std::io;

fn main() {
    let m = scanf()[1]; // Number of destined sum
    let array: Vec<u32> = scanf(); // Array of available cards
    let sum = search(array, m);
    println!("{}", sum);
}

fn search(array:Vec<u32>, m:u32) -> u32 {
    let mut max = 0;
    let len = array.len();
    for a in 0..len-2 {
        let card_a = array[a];
        for b in a+1..len-1 {
            let card_b = array[b];
            for c in b+1..len {
                let sum = card_a + card_b + array[c];
                if sum <= m && sum > max {
                    max = sum;
                }
            }
        }
    }
    max
}

fn scanf() -> Vec<u32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs: Vec<u32> = input.trim().split(" ").map(|x| x.parse().expect("Not an integer!")).collect();
    inputs
}