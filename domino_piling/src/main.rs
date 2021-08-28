use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut input = input.split_whitespace();

    let width: u32 = input.next().unwrap().parse().unwrap();
    let height: u32 = input.next().unwrap().parse().unwrap();

    let surface = width * height;

    let tiles_count = (surface as f32 / 2.0).floor();

    println!("{}", tiles_count);
}
