use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut input = input.split_whitespace();
    let mut number: u64 = input.next().unwrap().parse().unwrap();
    let reps: u64 = input.next().unwrap().parse().unwrap();

    for _ in 0..reps {
        if number % 10 == 0 {
            number /= 10;
        } else {
            number -= 1;
        }
    }

    println!("{}", number);
}
