use std::io;

fn main() {
    let mut statements_count = String::new();

    io::stdin().read_line(&mut statements_count).unwrap();

    let statements_count: u32 = statements_count.trim().parse().unwrap();

    let mut x = 0;

    for _ in 0..statements_count {
        let mut statement = String::new();

        io::stdin().read_line(&mut statement).unwrap();

        if statement.contains("++") {
            x += 1;
        } else {
            x -= 1;
        }
    }

    println!("{}", x);
}
