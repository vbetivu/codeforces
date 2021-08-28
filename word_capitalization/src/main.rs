use std::io;

fn main() {
    let mut word = String::new();

    io::stdin().read_line(&mut word).unwrap();

    println!("{}{}", &word[0..1].to_uppercase(), &word[1..])
}
