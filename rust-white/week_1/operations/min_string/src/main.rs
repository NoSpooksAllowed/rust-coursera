use text_io::read;

fn main() {
    let a: String = read!();
    let b: String = read!();
    let c: String = read!();

    if a < b && a < c {
        println!("{}", a);
    } else if b < c {
        println!("{}", b);
    } else {
        println!("{}", c);
    }
}
