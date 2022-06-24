use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = match File::open("src/errstats.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("src/errstats.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating a file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    println!("Reading file: {:?}", f);
}

/* How to deal with errors in Rust
Use match flow control instead of try-catch expression
Nested match control 
*/