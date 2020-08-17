use text_io::read;

fn main() {
    let a: i32 = read!();
    let b: i32 = read!();

    if b == 0 {
        println!("Impossible");
    } else {
        println!("{}", a / b);
    }
}
