use std::io::{self, Write};

fn main() {
    println!("## TODO LIST ##");

    let mut task_list:Vec<String> = Vec::new();
    // let mut task_list = Vec::new();

    // add_task(&mut task_list);
    show_menu();

}

fn show_menu(){
    loop{
        println!("1. Add new task");
        println!("2. Show tasks");
        println!("0. Exit");
        let _ = io::stdout().flush();

        let mut option = String::new();
        match io::stdin().read_line(&mut option) {
            Ok(_value) => {
                match option.as_str().trim() {
                    "1" => {
                        println!("Selected option 1");
                        continue;
                    },
                    "2" => {
                        println!("Selected option 2");
                        continue;
                    },
                    "0" => {break;},
                    &_ => {continue;}
                }
            },
            Err(error) => {
                println!("Error {}", error);
            }
        }
        
    }
}

fn add_task(task_list:&mut Vec<String>){
    loop {
        print!("Write a new task: ");
        let _ = io::stdout().flush();

        let mut task = String::new();
        match io::stdin().read_line(&mut task){
            Ok(_value) => {
                // println!("{}", task);
                // println!("{}", task.trim());

                //Claude session: `Error pushing trim() into a vector`
                task_list.push(task.trim().to_string());
            },
            Err(err) => {
                println!("Cannot read line: {}", err);
            }
        }

        show_tasks(&task_list);
    }
}

// Use slice &[String] instead of vectors Vec<String>
// https://dev.to/sharmaprash/why-is-it-discouraged-to-accept-string-vec-or-box-as-function-arguments-in-rust-3g72
fn show_tasks(task_items: &[String]) {
    for item in task_items {
        println!("{}", item);
    }
}
