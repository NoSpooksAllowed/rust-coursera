use text_io::read;

fn main() {
    let n: f64 = read!();
    let a: f64 = read!();
    let b: f64 = read!();
    let x: f64 = read!();
    let y: f64 = read!();
    let res: f64;

    if a < b {
        if n > a && n <= b {
            let discount = (n / 100.0) * x;

            res = n - discount;
        } else if n > b {
            let discount = (n / 100.0) * y;

            res = n - discount;
        } else {
            res = n;
        }

        println!("{}", res);
    }
}
