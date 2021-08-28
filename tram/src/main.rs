use std::io;

fn main() {
    let mut stops = String::new();

    io::stdin().read_line(&mut stops).unwrap();

    let stops: u32 = stops.trim().parse().unwrap();

    let mut capacity = 0;
    let mut current_fill = 0;

    let mut stop = String::new();

    for _ in 0..stops {
        io::stdin().read_line(&mut stop).unwrap();

        let mut stop_stats = stop.split_whitespace();

        let exit: i32 = stop_stats.next().unwrap().parse().unwrap();
        let enter: i32 = stop_stats.next().unwrap().parse().unwrap();

        current_fill += enter - exit;

        if current_fill > capacity {
            capacity = current_fill;
        }

        stop = String::default();
    }

    println!("{}", capacity);
}
