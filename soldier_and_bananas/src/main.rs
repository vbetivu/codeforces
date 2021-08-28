use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut input = input.split_ascii_whitespace();

    let cost: u64 = input.next().unwrap().parse().unwrap();
    let bank: u64 = input.next().unwrap().parse().unwrap();
    let amount: u64 = input.next().unwrap().parse().unwrap();

    let sum_of_all_numbers_until_n = (amount + 1) * amount;

    let price = cost as f64 * sum_of_all_numbers_until_n as f64 / 2.0;

    if price <= bank as f64 {
        println!("0",);
    } else {
        println!("{}", price - bank as f64);
    }
}
