use text_io::read;
use std::collections::HashMap;

fn main() {
    let num_of_iter: i32 = read!();

    for _i in 0..num_of_iter {
        let str1: String = read!();
        let str2: String = read!();

        if build_char_counters(&str1) == build_char_counters(&str2) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

fn build_char_counters(s: &str) -> HashMap<char, i32> {
    let mut res: HashMap<char, i32> = HashMap::new();

    for ch in s.chars() {
        if res.contains_key(&ch) {
            *res.get_mut(&ch).unwrap() += 1;
        } else {
            res.insert(ch, 1);
        } 
    }

    res
} 
