use std::io;

fn main() {
    let mut username = String::new();

    io::stdin().read_line(&mut username).unwrap();

    let mut unique_chars: Vec<char> = Vec::new();

    for char in username.chars() {
        if !unique_chars.contains(&char) {
            unique_chars.push(char);
        }
    }

    if unique_chars.len() % 2 == 0 {
        println!("CHAT WITH HER!");
    } else {
        println!("IGNORE HIM!");
    }
}
