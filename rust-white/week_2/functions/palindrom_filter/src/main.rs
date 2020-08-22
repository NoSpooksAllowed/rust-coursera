use text_io::read;

fn main() {
    let words = vec!["abacaba", "aba"];
    let min_length: i32 = read!();

    let res = palindrom_filter(&words, min_length);

    println!("{:?}", res);
}


fn is_palindrom(s: &str) -> bool {
    let half = s.len() / 2;

    s.bytes().take(half).eq(s.bytes().rev().take(half))
}

fn palindrom_filter(words: &Vec<&str>, min_length: i32) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    for word in words {
        if is_palindrom(word) && word.len() as i32 >= min_length {
            res.push(word.clone().to_string());
        }
    }

    res
}
