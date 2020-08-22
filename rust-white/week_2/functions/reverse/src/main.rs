fn main() {
    let mut numbers = vec![1, 5, 3, 4, 2];

    reverse(&mut numbers);

    println!("{:?}", numbers);
}

fn reverse(v: &mut Vec<i32>) {
    v.reverse(); // thats is it. So easy :)
}
