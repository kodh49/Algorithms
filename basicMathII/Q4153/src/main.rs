use std::io;

fn main() {
    loop {
        let mut looping = 0;
        let mut test_case: Vec<u32> = get_input();
        let iterator = test_case.iter();
        for item in iterator {
            if item == &0 {
                looping += 1;
            }
        }
        if looping >= 3 {
            break;
        } else {
            // Test if this case is a right triangle
            let result = check(&mut test_case);
            println!("{}", result);
        }
    }
}

fn get_input() -> Vec<u32> {
    let mut sides: Vec<u32> = Vec::new();
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Cannot read line");
    for item in line.split_whitespace() {
        match item.trim().parse() {
            Ok(num) => sides.push(num),
            Err(_) => sides.push(0),
        };
    }
    sides
}

fn check(list: &mut Vec<u32>) -> String {
    list.sort();
    let hypotenuse = list.pop().unwrap();
    let side_alpha = list.pop().unwrap();
    let side_beta = list.pop().unwrap();
    if hypotenuse.pow(2) == side_alpha.pow(2) + side_beta.pow(2) {
        let result = String::from("Right Triangle");
        result
    } else {
        let result = String::from("Not a right triangle");
        result
    }
}


/*

[Sample Inputs and Outputs]

1 2 3
Not a right triangle
3 4 5
Right Triangle
6 8 10
Right Triangle
5 12 13
Right Triangle
7 8 9
Not a right triangle
3 59 1
Not a right triangle
0 0 1
Not a right triangle
1 0 1
Right Triangle
0 1 0
Not a right triangle
0 0 0

*/