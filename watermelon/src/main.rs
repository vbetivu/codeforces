use std::io;

fn main() {
    let mut weight = String::new();

    io::stdin().read_line(&mut weight).unwrap();

    let weight: u32 = match weight.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    if weight <= 2 {
        println!("NO");
    } else {
        if weight % 2 == 0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
