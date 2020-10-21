use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let a = args[1].parse::<f64>().unwrap();
    let b = args[2].parse::<f64>().unwrap();
    let c = args[3].parse::<f64>().unwrap();

    let d = b * b - 4.0 * a * c;

    let x1 = (-b + d.sqrt()) / (2.0 * a);
    let x2 = (-b - d.sqrt()) / (2.0 * a);

    println!("{}", x1 as i32);
    println!("{}", x2 as i32);
}
