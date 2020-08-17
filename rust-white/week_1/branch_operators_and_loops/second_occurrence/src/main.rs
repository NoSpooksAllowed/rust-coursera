use text_io::read;

fn main() {
    let s: String = read!();
    let mut flag = -2;


    for (i, ch) in s.chars().enumerate() {
        if ch == 'f' && flag == -2 {
            flag = -1;
        } else if ch == 'f' && flag == -1 {
            flag = i as i32;
            break;
        }
    }

    println!("{}", flag);
}
