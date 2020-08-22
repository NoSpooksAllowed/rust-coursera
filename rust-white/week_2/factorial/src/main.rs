use text_io::read;

fn main() {
    let x: i32 = read!();

    println!("{}", factorial(x));
}

fn factorial(mut x: i32) -> i32 {
    let mut factorial = 1;

    if x < 0 {
        return factorial;
    }

    x %= 11;

    for i in 1..=x {
        factorial *= i;
    }

    return factorial;
}
