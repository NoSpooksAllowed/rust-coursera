use text_io::read;

fn main() {
    let operation_amount: i32 = read!();
    let mut queue_ppl: Vec<bool> = Vec::new();

    for _i in 0..operation_amount {
        let operation_name: String = read!();

        if operation_name == "WORRY" {

            let index: i32 = read!();
            queue_ppl.resize((index + 1) as usize, false);
            queue_ppl[index as usize] = true;
            
        } else if operation_name == "QUIET" {

            let index: i32 = read!();
            queue_ppl[index as usize] = false;

        } else if operation_name == "COME" {
            
            let index: i32 = read!();

            if index >= 0 {
                queue_ppl.resize(queue_ppl.len() + index as usize, false);

            } else {
                remove_last_elms(&mut queue_ppl, index);

            }
        } else if operation_name == "WORRY_COUNT" {

            let w_cnt = queue_ppl.iter().filter(|x| **x == true).count();
            println!("{}", w_cnt);
            
        }
    }
}

fn remove_last_elms(queue_ppl: &mut Vec<bool>, amount: i32) {
    for _i in 0..amount.abs() {
        queue_ppl.pop();
    }
}
