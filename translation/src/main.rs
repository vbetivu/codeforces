use std::io;

fn main() {
    let mut first_word = String::new();

    io::stdin().read_line(&mut first_word).unwrap();

    let mut second_word = String::new();

    io::stdin().read_line(&mut second_word).unwrap();

    let mut second_word = second_word.trim().split("").collect::<Vec<&str>>();
    second_word.reverse();

    let second_word = second_word.join("");

    if first_word.trim() == second_word {
        println!("YES");
    } else {
        println!("NO");
    }
}
