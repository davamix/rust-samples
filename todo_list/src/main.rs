use std::io::{self, Write};

fn main() {
    println!("## TODO LIST ##");

    let mut task_list = Vec::new();

    loop {
        print!("Write a new task: ");
        let _ = io::stdout().flush();

        let mut task = String::new();
        let _ = io::stdin().read_line(&mut task);

        task_list.push(task);

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
