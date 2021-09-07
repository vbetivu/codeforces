use std::io;

fn main() {
    let mut number = String::new();

    io::stdin().read_line(&mut number).unwrap();

    let digits = number.trim().split("");

    let mut lucky_digits_count = 0;

    for digit in digits {
        if digit == "7" || digit == "4" {
            lucky_digits_count += 1;
        }
    }

    if lucky_digits_count == 4 || lucky_digits_count == 7 {
        println!("YES");
    } else {
        println!("NO");
    }
}
