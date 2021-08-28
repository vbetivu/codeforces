use std::io;

fn main() {
    let mut first_word = String::new();
    let mut second_word = String::new();

    io::stdin().read_line(&mut first_word).unwrap();
    io::stdin().read_line(&mut second_word).unwrap();

    let first_word = first_word.trim().to_ascii_lowercase();
    let second_word = second_word.trim().to_ascii_lowercase();

    if first_word > second_word {
        println!("1");
    } else if second_word > first_word {
        println!("-1");
    } else {
        println!("0");
    }
}
