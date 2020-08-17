use text_io::read;

fn main() {
    let mut n: i32 = read!();
    let mut binary_n = Vec::new();

    while n > 0 {
        binary_n.push(n % 2);
        n /= 2;
    }

    for i in binary_n.iter().rev() {
        print!("{}", i);
    }

    println!();
}
