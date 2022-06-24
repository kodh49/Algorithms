fn main() {
    vector_test();
    multi_vector_test();
}

#[derive(Debug)]
enum PACKET {
    Bytes(u64),
    IPv4(String),
    IcmpSeq(u32),
    Ttl(u32),
    Time(f64),
}

fn multi_vector_test() {
    let ping_info = vec![
        PACKET::Bytes(64),
        PACKET::IPv4(String::from("172.217.175.68")),
        PACKET::IcmpSeq(1),
        PACKET::Ttl(115),
        PACKET::Time(36.8)
    ];
    for var in ping_info {
        print!("{:?} | ", var);
    }
}

// Using Collections(1): Vectors (collect lists of single-type values)
fn vector_test() {
    // A vector that is filled with 2, 3, 4, and 5; Without annotation, default would be i32
    let mut int_vector = vec![2,3,4,5];
    
    {
        // An empty one
        let mut unsigned_vector: Vec<u32> = Vec::new();
        unsigned_vector.push(20);
    }
    // Once a vector goes out of the scope, all of its values will be dropped too
    // unsigned_vector.push(0); // Compile Error: cannot find value `unsigned_vector` in this scope

    // Add a value to the vector
    int_vector.push(100);
    // Delete a value from the vector
    int_vector.pop();
    
    // Read Values from the vector
    let first: &i32 = &int_vector[0];
    println!("First element is : {}", first);
    for index in 0..5 {
        let element = match int_vector.get(index) {
            Some(first) => *first,
            Some(second) => *second,
            Some(third) => *third,
            Some(fourth) => *fourth,
            None => {
                println!("Index out of bound");
                -1
            },
        };
        println!("{}th element= {}", index, element);
    }
}


// fn get_input() -> String {
//     let mut input = String::new();
//     io::stdin()
//     .read_line(&mut input)
//     .expect("Failed to read line");
//     let input: String = match input.trim().parse() {
//         Ok(string) => string,
//         Err(_) => String::from(""),
//     };
//     input
// }

// fn write_line(msg: String) {
//     println!("{}", msg);
// }