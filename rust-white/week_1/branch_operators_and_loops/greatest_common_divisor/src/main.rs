use::text_io::read;

fn main() {
    let a: i32 = read!();
    let b: i32 = read!();

    println!("{}", gcd(a, b));
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
