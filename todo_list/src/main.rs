use std::io::{self, Write};

fn main() {
    println!("## TODO LIST ##");

    let mut task_list:Vec<String> = Vec::new();
    // let mut task_list = Vec::new();

    // add_task(&mut task_list);
    loop{
        // let mut option = String::new();
        // option = show_menu();

        match show_menu().as_str().trim() {
            "1" => {
                add_task(&mut task_list);
            },
            "2" => {
                show_tasks(&task_list);
            },
            "0" => {break;},
            &_ => {continue;}
        }
    }
}

fn show_menu() -> String {
        println!("1. Add new task");
        println!("2. Show tasks");
        println!("0. Exit");
        let _ = io::stdout().flush();

        let mut option = String::new();
        let _ = io::stdin().read_line(&mut option);

        return option.to_string();
}

fn add_task(task_list:&mut Vec<String>){
    println!("Leave empty and press Enter to go back to menu");

    loop {
        print!("-> Task: ");
        let _ = io::stdout().flush();

        let mut task = String::new();
        match io::stdin().read_line(&mut task){
            Ok(_value) => {
                if task.trim().is_empty() {
                    break;
                }

                //Claude session: `Error pushing trim() into a vector`
                task_list.push(task.trim().to_string());
            },
            Err(err) => {
                println!("Cannot read line: {}", err);
            }
        }
    }
}

// Use slice &[String] instead of vectors Vec<String>
// https://dev.to/sharmaprash/why-is-it-discouraged-to-accept-string-vec-or-box-as-function-arguments-in-rust-3g72
fn show_tasks(task_items: &[String]) {
    println!("## List of all tasks ##");

    for item in task_items {
        println!("{}", item);
    }
}
