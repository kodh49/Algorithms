use std::collections::HashMap;
use std::io::stdin;

fn main() {
    help();
    let mut database: HashMap<String, String> = HashMap::new();
    loop {
        // Write functions for getting input
        let cmdline = read_data();
        // Use match-arms to execute each command
        match cmdline.get(0).unwrap().as_ref() {
            // inquire (* or department)
            "inquire" => inquire_data(&database, cmdline.get(1).unwrap().to_string()),
            // add (subject) to (object)
            "add" => add_data(&mut database, cmdline.get(1).unwrap(), cmdline.get(3).unwrap()),
            "help" => help(),
            _ => println!(">> Unknown command! Please type 'help' for available system commands"),
        }
        println!("");
    }
}

// This function will return a vector that contains each command of the entire line
pub fn read_data() -> Vec<String> {
    let mut msg = String::new();
    stdin().read_line(&mut msg).expect("Cannot read line!");
    let mut command_line: Vec<String> = Vec::new();
    for keyword in msg.trim().split_whitespace() {
        command_line.push(keyword.to_string());
    }
    command_line
}

fn inquire_data(map: &HashMap<String, String>, loc: String) {
    match loc.as_str() {
        "*" => {
            // Iterate over all the key-value sets
            let mut teamlist: Vec<String> = Vec::new();
            for (_key, value) in map {
                if !teamlist.contains(value) {
                    teamlist.push(value.to_string());
                }
            }
            teamlist.sort();
            for department in teamlist {
                print_name_order(map, department);
            }
        },
        _ => print_name_order(map, loc),
    }
}

fn print_name_order(map: &HashMap<String, String>, loc: String) {
    let mut namelist: Vec<String> = Vec::new();
    // Iterate over the keys which value is equal to 'loc'
    for (key, value) in map {
        if *value == loc.to_string() {
            // sort KEYS
            namelist.push(key.to_string());
        }
    }
    // Sort the vector "namelist" in alphabetical order
    namelist.sort();
    // Print entire list
    println!("===== {} Department =====", loc);
    for name in namelist {
        println!("{}", name);
    }
    println!("");
}

fn help() {
    let start_msg = ">> Available System Commands\n
    - add (name) to (department)\n
    - inquire *\n
    - inquire (department)\n";
    println!("{}", start_msg);
}


fn add_data(map: &mut HashMap<String, String>, from: &String, to: &String) {
    map.entry(from.to_string()).or_insert(to.to_string());
    println!(">> Successfully added {} to {}", from, to);
}

/*
# What to have in this program
Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
For example, “Add Sally to Engineering” or “Add Amir to Sales.”
Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
*/
