use std::io;

fn main() {
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    let mut queue = String::new();

    io::stdin().read_line(&mut queue).unwrap();

    let mut input_line = input_line.trim().split_ascii_whitespace();
    input_line.next().unwrap();
    let time: i32 = input_line.next().unwrap().parse().unwrap();

    let mut queue: Vec<&str> = queue.trim().split("").collect();

    for _ in 0..time {
        let mut j = 0;

        while j < queue.len() - 1 {
            if queue[j] == "B" && queue[j + 1] == "G" {
                let temp = queue[j];
                queue[j] = queue[j + 1];
                queue[j + 1] = temp;

                j += 2;
            } else {
                j += 1;
            }
        }
    }

    println!("{}", queue.join(""));
}
