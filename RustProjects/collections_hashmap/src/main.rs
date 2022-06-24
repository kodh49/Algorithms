use std::collections::HashMap;

fn main() {
    // create an empty hashmap
    let mut scores: HashMap<String, u32> = HashMap::new();
    // Add a set of key & value to a hashmap
    scores.insert(String::from("Blue"), 100);
    scores.insert(String::from("Red"), 200);

    // Ownership *
    // For those types that implement copy-traits, the data(&type) is directly copied to the hashmap and be available after assignment
    // However, heap-spaced data such as String, Vector, or etc. give their ownership to the hashmap and no longer be able to be accessed later on

    // Create a list of keys that'll later used to hold a hashmap
    let pc_name = vec!["Sentinel", "Manticore", "Pleiades", "Nittany", "Dragonglass", "Krypton"];
    // Create a list of values that'll later used to hold a hashmap
    let ipv4 = vec!["6.228.209.2", "174.133.46.85", "12.188.252.121", "85.189.190.198", "130.7.60.236", "199.190.64.47"];
    // Create a hashmap with initial set of keys and values
    let nameaddress: HashMap<_, _> = pc_name.into_iter().zip(ipv4.into_iter()).collect();

    // Accessing values in a hashmap
    let score_blue = match scores.get(&String::from("Blue")) {
        Some(inner) => inner,
        None => &0,
    };
    let score_red = match scores.get(&String::from("Red")) {
        Some(inner) => inner,
        None => &0,
    };
    println!("{}", format!("{} vs {}", score_blue, score_red));

    // Iterating over a hashmap
    for (key, value) in &nameaddress {
        // Accessing the key-value set in random order
        println!("{}: {}", key, value);
    }

    // Updating a hashmap
    let mut flight: HashMap<String, u32> = HashMap::new();
    flight.insert(String::from("DELTA-A"), 1754111);
    flight.insert(String::from("UNITED-A"), 1859270);
    flight.insert(String::from("DELTA-B"), 1482890);
    flight.insert(String::from("DELTA-C"), 1737510);

    // In order to replace(overwrite) the value, just use another 'insert' keyword with destined key
    flight.insert(String::from("UNITED-A"), 1000000);
    let united_price = flight.get("UNITED-A").unwrap();
    println!("UNITED-A Price: {}", united_price);

    // In order to insert a value only when the key has no value, entry() && or_insert() keyword together
    flight.entry(String::from("DELTA-D")).or_insert(2819535); // This will only put 2819535 into the value that is associated with the key "DELTA-D"
    println!("{:?}", flight); // DELTA-D will be added to the hashmap because the key does not exist at the compile time

    // In order to update a value based on the old value, 
    let text = "Speak your mind. You are as worried as the sum of yourself and the
    difference between my small smooth hamster and my nose. Speak your
    mind! You are as brave as the sum of your fat little stuffed misused dusty
    old rotten codpiece and a beautiful fair warm peaceful sunny summer's
    day. You are as healthy as the difference between the sum of the
    sweetest reddest rose and my father and yourself! Speak your mind!
    Thou art as loving as the product of the bluest clearest sweetest sky
    and the sum of a squirrel and a white horse. Thou art as beautiful as
    the difference between Juliet and thyself. Speak thy mind!";
    
    let mut wordcount = HashMap::new();
    for word in text.split_whitespace() {
        let count = wordcount.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", wordcount);
}
