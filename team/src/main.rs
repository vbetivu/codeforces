use std::io;

fn main() {
    let mut count = String::new();
    io::stdin().read_line(&mut count).unwrap();

    let count: usize = count.trim().parse().unwrap();

    let mut result = 0;

    for _ in 0..count {
        let mut decision_line = String::new();
        let mut decision: u8 = 0;

        io::stdin().read_line(&mut decision_line).unwrap();

        let mut decision_iterator = decision_line.split_whitespace();

        for _ in 0..3 {
            if decision_iterator.next().unwrap() == "1" {
                decision += 1;
            }
        }

        if decision > 1 {
            result += 1;
        }
    }

    println!("{}", result);
}
