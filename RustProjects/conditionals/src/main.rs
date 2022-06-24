fn main() {
    test_assign_if();
    test_for();
    test_if();
    test_loop();
    test_while();
    test_for_range();
}

// if: Similar to other programming languages
fn test_if() {
    let number = 1275;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    }
    else if number % 3 == 0 {
        println!("Number is divisible by 3");
    }
    else if number % 2 == 0 {
        println!("Number is divisible by 2");
    }
    else {
        println!("Number is a prime");
    }
}

// if: Rust enables programmer to assign the result of if/else
fn test_assign_if() {
    let tf = true;
    let number = if tf {7} else {19};
    println!("{}",number);
}

// loop: infinite as default, 'break' to terminate manually
fn test_loop() {
    let mut itr = 0;
    let result = loop {
        if itr == 100000000 {
            break itr;
        }
        else {
            itr = itr + 1;
        }
    };
    println!("result: {}", result);
}

// while: repeat regarding the given condition
fn test_while() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number-=1;
    };
    println!("LiftOff!!!");
}

// for
fn test_for() {
    let arr = [10 ,-20, 30, -40, 50, -60];
    for element in arr.iter() {
        println!("{}", element);
    }
}

// for - range
fn test_for_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
}