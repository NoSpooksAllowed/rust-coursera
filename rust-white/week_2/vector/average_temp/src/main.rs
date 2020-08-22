use text_io::read;

fn main() {
    let vector_max_size: usize = read!();
    let mut temps = vec![0; vector_max_size];

    fill_vector_temp(&mut temps);
    
    let average_temp = count_average_temp(&temps);

    let higher_average = count_higher_average(&temps, average_temp);

    println!("{}", higher_average.len());
    println!("{:?}", higher_average);

}

fn fill_vector_temp(v: &mut Vec<i32>) {
    for item in v {
        *item = read!();
    }
}

fn count_average_temp(v: &Vec<i32>) -> i32 {
    let mut res = 0;

    for n in v {
        res += n;
    }

    return res / v.len() as i32;
}

fn count_higher_average(v: &Vec<i32>, average_temp: i32) -> Vec<i32> {
    let mut higher_average = Vec::new();

    for i in 0..v.len() {
        if v[i] > average_temp {
            higher_average.push(i as i32);
        }
    }

    return higher_average;
}
