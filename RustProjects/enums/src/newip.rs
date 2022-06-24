fn execute() {
    let computer_a = IPAddress {
        name: String::from("DESKTOP-BVKSQOR"),
        address: String::from("115.109.74.123"),
        iptype: IPKind::IPv4,
        active: false,
    };
    let computer_b = IPAddress {
        name: String::from("DESKTOP-QOCZJDH"),
        address: String::from("145.184.89.84"),
        iptype: IPKind::IPv6,
        active: true,
    };
    println!("COMPUTER A = {:?}", computer_a);
    println!("COMPUTER B = {:?}", computer_b);
}

#[derive (Debug)]
enum IPKind {
    IPv4,
    IPv6,
}

#[derive (Debug)]
struct IPAddress {
    name: String,
    address: String,
    iptype: IPKind,
    active: bool,
}