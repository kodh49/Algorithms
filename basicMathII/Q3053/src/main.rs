use std::io;

fn main() {
    let radius: f64 = get_input();
    let euclidean: f64 = get_euclidean(&radius);
    let taxi: f64 = get_taxi(&radius);
    println!("{}\n{}", euclidean, taxi);
}

// Sample input: 1 ~ 10,000 Positive integer
fn get_input() -> f64 {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).expect("Cannot read from line");
    let radius: f64 = match line.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    radius
}

// Get the size of an circle based on Euclidean Geometry
fn get_euclidean(radius: &f64) -> f64 {
    3.141593*radius*radius // pi*radius^2
}

// Get the size of an circle based on Non-Euclidean Geometry
fn get_taxi(radius: &f64) -> f64 {
    2.0*radius*radius
}