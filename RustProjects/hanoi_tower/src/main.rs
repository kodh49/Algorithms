use std::io::{self, Write, BufWriter};


fn main() {
    let disk = scanf();
    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut buff = BufWriter::new(lock);
    buff.write_fmt(format_args!("{}\n", 2_u32.pow(disk) - 1)).expect("Failed to write");
    move_disks(1, 3, 2, disk);
}

fn scanf() -> u32 {
    let mut msg = String::new();
    io::stdin().read_line(&mut msg).expect("Cannot Read Line");
    let msg:u32 = match msg.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    msg
}

fn move_disks(from:u8, to:u8, pass:u8, disks:u32) {
    if disks == 2 {
        print!("{} {}\n", from, pass);
        print!("{} {}\n", from, to);
        print!("{} {}\n", pass, to);
        io::stdout().flush().unwrap();
    } else {
        move_disks(from, pass, to, disks-1);
        print!("{} {}\n", from, to);
        io::stdout().flush().unwrap();
        move_disks(pass, to, from, disks-1);
    }
}