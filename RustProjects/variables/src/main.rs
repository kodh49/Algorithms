fn main() {
    use_variables();
    use_tuple();
    use_character();
    use_array();
    get_int_range(8);
}

// Rust sets variable immutable as default
fn use_variables() {
    let mut x = 5;
    const SPEED_OF_LIGHT: u64 = 300_000_000_000;
    println!("X: {}", x);
    x = 6;
    println!("X: {}", x);
    println!("Speed of Light = {}", SPEED_OF_LIGHT);
}

fn use_array() {
    let arr_a = [1,2,3,4,5,6,7,8,9,10];
    let arr_b: [i32; 5] = [-1, -2, -3, -4, -5];
    let arr_c = [0; 8];
    let numsets = ["Natural", "Integer", "Rationals", "Reals", "Complexes"];
    // how to access elements of the array
    println!("{}, {}, {}, {}", arr_a[0], arr_a[1], arr_a[2], arr_a[3]);
    println!("{:?}", arr_b);
    println!("{:?}", arr_c);
    println!("{:?}", numsets);
    // The index itself can be mutable
}

fn use_tuple() {
    let tup: (i32, f64, u8) = (500, 6.42, 1);
    let (x, y, z) = tup;
    println!("Tuple: ({}, {}, {})", x, y, z);
    println!("Tuple Indexing: {} {} {}", tup.2, tup.1, tup.0);
}

fn use_character() {
    let c = 'z';
    let z = 'Z';
    let kor_kiyuk = 'ㄱ';
    let kor_nieun = 'ㄴ';
    let heart_eyed_cat = '😻';
    println!("{}, {}, {}, {}, {}", c, z, kor_kiyuk, kor_nieun, heart_eyed_cat);
}


// Rust enables shadowing
fn get_int_range(n:u32) {
    let num:u64 = 2;
    let num = num.pow(n)-1;
    println!("{}", num);
}