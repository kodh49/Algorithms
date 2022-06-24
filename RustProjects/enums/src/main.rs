fn main() {
    let computer_a = String::from("DESKTOP-BVKSQOR");
    let computer_a = IPAddress::IPv4(computer_a, 115, 109, 74, 123, true);
    let computer_b = String::from("DESKTOP-QOCZJDH");
    let computer_b = IPAddress::IPv6(computer_b, 145, 184, 89, 84, 0, 1, false);
    println!("COMPUTER A = {:?}", computer_a);
    println!("COMPUTER B = {:?}", computer_b);
}

// this code executes in the same way newip.rs does
// The difference is that main.rs does include struct inside of the enum unlike newip.rs
#[derive (Debug)]
enum IPAddress {
    IPv4(String, u8, u8, u8, u8, bool),
    IPv6(String, u8, u8, u8, u8, u8, u8, bool),
}

enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    Color(i8, i8, i8),
}

/* What would happen if we assign a function which requires enum as parameters? */