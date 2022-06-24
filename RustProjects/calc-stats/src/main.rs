use std::io;
use std::collections::HashMap;

fn main() {
    let mut list: Vec<f64> = Vec::new();
    let input = read_data();
    for num in input.split_whitespace() {
        let num: f64 = num.parse::<f64>().unwrap();
        list.push(num);
    }
    // Calculate the mean of the list
    let mean = get_mean(&list);
    // Calculate the mode of the list
    let mode = get_mode(&list);
    // Calculate the median of the list
    let median = get_median(&mut list);
    println!("Mean: {} | Mode: {:?} | Median: {}", mean, mode, median);
}

pub fn read_data() -> String {
    println!("Type in elements of the list");
    let mut uin = String::new();
    io::stdin().read_line(&mut uin).expect("Cannot read line");
    uin
}

pub fn get_mean(list:&Vec<f64>) -> f64 {
    let mut mean: f64 = 0.0;
    let mut count = 0.0;
    for number in list {
        mean += number;
        count += 1.0;
    }
    mean/count
}

pub fn get_mode(list:&Vec<f64>) -> Vec<String> {
    let mut dictionary = HashMap::new();
    for number in list {
        let count = dictionary.entry(number.to_string()).or_insert(0);
        *count += 1;
    }
    // check for the greatest value
    let mut mode = MODE {
        key: Vec::new(),
        appearance: 1,
    };
    for (key, value) in dictionary {
        if value >= 2 {
            if value > mode.appearance {
                mode.key.insert(0, key);
                mode.appearance = value;
            }
            else {
                mode.key.push(key);
            }
        }
    }
    if mode.key.len() == 0 {
        mode.key.push(String::from("None"));
    }
    mode.key
}

struct MODE {
    key: Vec<String>,
    appearance: i32,
}

pub fn get_median(list:&mut Vec<f64>) -> f64 {
    // insertion sort
    let len = list.len();
    for index in 1..len {
        // index -> Put this in the right order
        for cmp in 0..index {
            println!("index={}, cmp={}", index, cmp);
            if list.get(index).unwrap() < list.get(cmp).unwrap() {
                println!("ㄴ Swaped!");
                list.swap(cmp, index);
            }
        }
    }
    println!("{:?}", list);
    // indexing the middle
    if len%2 == 0 {
        let median = (list.get(len/2).unwrap()+list.get((len/2)-1).unwrap())/2.0;
        median
    }
    else {
        let median = *list.get(len/2).unwrap();
        median
    }
}

/* What to achieve in this program
Given a list of integers, use a vector and return the
    - mean (the average value),
    - median (when sorted, the value in the middle position), and
    - mode (the value that occurs most often; a hash map will be helpful here)
of the list.
*/