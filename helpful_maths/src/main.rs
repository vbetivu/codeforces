use std::io;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).unwrap();

    let mut line: Vec<&str> = line.trim_end().split('+').collect();

    line.sort_unstable();

    println!("{}", line.join("+"));
}
