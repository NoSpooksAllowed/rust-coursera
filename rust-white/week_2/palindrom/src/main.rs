use text_io::read;

fn main() {
    let word: String = read!();

    println!("{}", is_palindrom(&word));
}

fn is_palindrom(s: &String) -> bool {
    let half = s.len() / 2;

    s.bytes().take(half).eq(s.bytes().rev().take(half))
}
