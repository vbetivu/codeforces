use std::io;

fn main() {
    let mut word = String::new();

    io::stdin().read_line(&mut word).unwrap();

    let word = word.trim();

    let mut lowercase_count = 0;
    let mut uppercase_count = 0;

    for ch in word.chars() {
        if ch.is_lowercase() {
            lowercase_count += 1;
        } else {
            uppercase_count += 1;
        }
    }

    if uppercase_count > lowercase_count {
        println!("{}", word.to_uppercase());
    } else {
        println!("{}", word.to_lowercase());
    }
}
