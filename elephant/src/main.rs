use std::io;

fn main() {
    let mut distance = String::new();

    io::stdin().read_line(&mut distance).unwrap();

    let distance: u32 = distance.trim().parse().unwrap();

    if distance <= 5 {
        println!("1");
    } else if distance % 5 == 0 {
        println!("{}", distance / 5);
    } else {
        println!("{}", distance / 5 + 1);
    }
}
