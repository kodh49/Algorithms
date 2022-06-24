fn main() {
    // function_d(216, 28705, 'Z');
    // function_c(3.14159, false);
    // function_b(128);
    // function_a();
    println!("{}", add(1, 5));
    println!("{}", sub(2, 4));
    println!("{}", mul(10, 3));
    println!("{}", quo(2, 1));
    println!("{}", rem(0, 2));
}

fn add(x:i32, y:i32) -> i32 {
    x + y
}

fn sub(x:i32, y:i32) -> i32 {
    x - y
}

fn mul(x:i32, y:i32) -> i32 {
    let x = x + 1;
    let y = y + 3;
    x * y
}

fn quo(x:i32, y:i32) -> i32 {
    x / y
}

fn rem(x:i32, y:i32) -> i32 {
    x % y
}

// fn function_a() {
//     println!("No parameter has passed through function_a.");
// }

// fn function_b(x:i32) {
//     println!("One parameter({}) has passed through function_b.", x);
// }

// fn function_c(x:f64, y:bool) {
//     println!("Two pxrameters({}, {}) have passed through function_c.", x, y);
// }

// fn function_d(x:i16, y:i32, z:char) {
//     println!("Three parameters({}, {}, {}) have passed through function_d.", x, y, z);
// }


/* VSC Code Shortcuts
Ctrl + D -> Select all the forms that has a same name
Alt + Click -> Add cursor at the clicked point
Alt + Up/Down -> Move the specific line up and down
ALt + Shift + Up/Down -> Copy the line and paste it above/below
Ctrl + / -> Comment the selected code area
Alt + SHift + I -> Get cursor for every line selected
*/