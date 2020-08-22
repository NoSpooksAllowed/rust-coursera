use text_io::read;

fn main() {
    let a: i32 = read!();
    let mut b: i32 = read!();

    update_if_greater(a, &mut b);

    println!("{}", b);
}

fn update_if_greater(first: i32, second: &mut i32) {

    if first > *second {
        *second = first;
    }
}
