fn main() {
    with_boilerplates();
    without_boilerplates();
}

fn with_boilerplates () {
    let target_u8 = Some(253);
    match target_u8 {
        Some(53) => println!("Two Hundred Fifty Three"),
        _ => (),
    }
}

fn without_boilerplates () {
    let target_u8 = Some(143);
    if let Some(133) = target_u8 {
        println!("One Hundred Fourty Three");
    }
}