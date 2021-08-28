use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut input = input.trim_end().split_whitespace();

    let mut limak_weight: u64 = input.next().unwrap().parse().unwrap();
    let mut bob_weight: u64 = input.next().unwrap().parse().unwrap();

    let mut years = 0;

    while limak_weight <= bob_weight {
        limak_weight *= 3;
        bob_weight *= 2;
        years += 1;
    }

    println!("{}", years);
}
