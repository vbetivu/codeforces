use std::io;

fn main() {
    let mut lines_number = String::new();

    io::stdin().read_line(&mut lines_number).unwrap();

    let lines_number: u32 = match lines_number.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    let mut words: Vec<String> = Vec::with_capacity(lines_number as usize);

    for _ in 0..lines_number {
        let mut word = String::new();
        io::stdin().read_line(&mut word).unwrap();

        words.push(word.trim().to_string());
    }

    for word in words.iter() {
        let len = word.len();

        if len > 10 {
            println!(
                "{}{}{}",
                word.chars().nth(0).unwrap(),
                len - 2,
                word.chars().nth(len - 1).unwrap()
            )
        } else {
            println!("{}", word)
        }
    }
}
