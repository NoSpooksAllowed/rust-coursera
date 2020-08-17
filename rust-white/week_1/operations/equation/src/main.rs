use text_io::read;

fn main() {
    let a: f64 = read!();
    let b: f64 = read!();
    let c: f64 = read!();

    if a == 0.0 && b == 0.0 {
        
        println!(" ");
    } else if a == 0.0 {
        let x = -c / b;

        println!("{}", x);
    } else {
        let d = b.powf(2.0) - 4.0 * a * c;

        if d == 0.0 {
            let x = -b / (2.0 * a);

            println!("{}", x);
        } else if d < 0.0 {
            println!(" ");
        } else {
            let x1 = (-b + d.sqrt()) / (2.0 * a);
            let x2 = (-b - d.sqrt()) / (2.0 * a);

            println!("{} {}", x1, x2);
        }
    }
}
