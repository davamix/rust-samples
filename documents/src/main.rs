use std::io::{self, Write};
use std::fs;

const FILE_NAME: &str = "test_file.txt";

fn main() {
    loop {
        match show_menu().as_str() {
            "1" => read_file(),
            "2" => write_file(),
            "0" => break,
            _ => continue //,
        }
    }
}

fn show_menu() -> String {
    println!("1. Read file");
    println!("2. Write file");
    println!("0. Exit");
    print!("-> ");
    let _ = io::stdout().flush();

    let mut option = String::new();

    match io::stdin().read_line(&mut option) {
        Ok(_value) => {
            return option.trim().to_string();
        },
        Err(error) => {
            println!("Cannot read the value {}", error);
        }
    }
    
    return Default::default();
}

fn write_file() {
    let mut text = String::new();
    print!("Content -> ");
    let _ = io::stdout().flush();

    match io::stdin().read_line(&mut text) {
        Ok(value) => {
            match fs::write(FILE_NAME, text.trim()) {
                Ok(_) => {
                    println!("File saved on {}", FILE_NAME);
                },
                Err(error) => {
                    println!("Cannot write file: {}", error);
                }
            }
        },
        Err(error) => {
            println!("Cannot read the line: {}", error);
        }
    }
}

fn read_file() {
    match fs::read_to_string(FILE_NAME) {
        Ok(content) => {
            println!("File content: {}", content);
        },
        Err(error) => {
            println!("Cannot read file: {}", error);
        }
    }
}
