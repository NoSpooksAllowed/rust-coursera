use::text_io::read;

fn main() {
    let days_in_months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    
    let number_of_commands: i32;
    let mut name_of_command: String;
    let mut index: usize;
    let mut current_month = 0;
    let mut name_of_task: String;

    let mut tasks: Vec<Vec<String>> = Vec::new();

    for _i in 0..days_in_months[current_month] {
        tasks.push(Vec::new());
    }

    number_of_commands = read!();

    for _i in 0..number_of_commands {
        name_of_command = read!();

        if name_of_command == "ADD" {
            index = read!();
            name_of_task = read!();
            tasks[index - 1].push(name_of_task);

        } else if name_of_command == "DUMP" {
            index = read!();
            print!("{} ", tasks[index - 1].len());

            for word in &tasks[index - 1] {
                print!("{} ", word);
            }

            println!();

        } else if name_of_command == "NEXT" {
            current_month = (current_month + 1) % 12; 
            let last_day = days_in_months[current_month] - 1;
            let previous_month = days_in_months[(current_month + 11) % 12];

            for i in days_in_months[current_month]..previous_month {
                let end = tasks[last_day].len();
                let prev_month_tasks = tasks[i].clone();
                tasks[last_day].splice(end.., prev_month_tasks.iter().cloned());
                tasks[i].clear();
            }
        }
    }
}
