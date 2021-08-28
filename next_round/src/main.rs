use std::io;

fn main() {
    let mut f_line = String::new();

    io::stdin().read_line(&mut f_line).unwrap();

    let lowest_score_index: usize = f_line.split_whitespace().last().unwrap().parse().unwrap();

    let mut scores = String::new();

    io::stdin().read_line(&mut scores).unwrap();

    let scores: Vec<u32> = scores
        .split_whitespace()
        .map(|item| item.parse().unwrap())
        .collect();

    let min_score = scores.get(lowest_score_index - 1).unwrap().clone();
    let mut total_passed = 0;

    for item in scores {
        if item > 0 && item >= min_score {
            total_passed += 1;
        }
    }

    println!("{}", total_passed)
}
