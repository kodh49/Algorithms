use std::io;

fn main() {
    println!("Input the temperature in Fahrenheit");
    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");
    let fahrenheit:f32 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    let result = to_celsius(fahrenheit);
    println!("in Celsius: {}", result);
}

// Convert temperatures between Fahrenheit and Celsius.
fn to_celsius(fh:f32) -> f32 {
    let fh:f32 = (fh - 32.0)*5.0/9.0;
    fh
}