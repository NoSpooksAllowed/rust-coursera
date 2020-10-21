use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let n = args[1].parse::<i32>().unwrap();
    for i in 0..n {

        for _j in i..n-1 {
            print!(" ");
        }

        for _x in 0..i+1 {
            print!("#");
        }
        println!();
    }
}
