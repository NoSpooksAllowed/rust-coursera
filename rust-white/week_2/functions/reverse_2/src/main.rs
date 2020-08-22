fn main() {
    let numbers = vec![1, 5, 3, 4, 2]; 

    println!("{:?}", reverse(&numbers));
}

fn reverse(v: &Vec<i32>) -> Vec<i32> {
    let mut reversed = v.clone();

    reversed.reverse();

    reversed
}
