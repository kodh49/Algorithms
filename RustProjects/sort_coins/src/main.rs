fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;
    let cents_penny = value_in_cents(penny);
    let cents_nickel = value_in_cents(nickel);
    let cents_dime = value_in_cents(dime);
    let cents_quarter = value_in_cents(quarter);
    println!("You've got {}, {}, {}, {} Cents, respectively.", cents_penny, cents_nickel, cents_dime, cents_quarter);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}