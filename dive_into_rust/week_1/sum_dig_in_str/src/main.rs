use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut sum = 0;

    for ch in args[1].chars() {
        sum += ch.to_digit(10).unwrap();
    }

    println!("{}", sum);
}
