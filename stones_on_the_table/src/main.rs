use std::io;

fn main() {
    io::stdin().read_line(&mut String::new()).unwrap();

    let mut sequence = String::new();

    io::stdin().read_line(&mut sequence).unwrap();

    let mut result = 0;

    let mut sequence = sequence.chars();
    let mut prev = sequence.next().unwrap();

    for char in sequence {
        if prev == char {
            result += 1;
        } else {
            prev = char;
        }
    }

    println!("{}", result);
}
