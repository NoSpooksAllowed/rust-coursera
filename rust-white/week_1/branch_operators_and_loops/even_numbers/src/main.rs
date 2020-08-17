use text_io::read;

fn main() {
    let a: i32 = read!();
    let b: i32 = read!();

    for i in a..=b {
        if i % 2 == 0 {
            print!("{} ", i);
        }
    }

    println!();
}
