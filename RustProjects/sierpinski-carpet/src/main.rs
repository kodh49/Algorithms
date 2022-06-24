use std::io;

fn main() {
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("Failed to read line");
    let i:f64 = match i.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    let i:u32 = i.log(3.0) as u32;
    println!("{}", sierpinski_carpet(i));
}
 
fn sierpinski_carpet(n: u32) -> String {
    let mut carpet = vec!["*".to_string()];
    for _ in 0..n {
        let mut top: Vec<_> = carpet.iter().map(|x| x.repeat(3)).collect();
        let middle: Vec<_> = carpet
            .iter()
            .map(|x| x.to_string() + &x.replace("*", " ") + x)
            .collect();
        let bottom = top.clone();
 
        top.extend(middle);
        top.extend(bottom);
        carpet = top;
    }
    carpet.join("\n")
}
 