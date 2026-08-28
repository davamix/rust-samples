use std::io::{self, Write};

fn main() {
    print!("Gess number:");
    let _ = io::stdout().flush();
    
    let mut number = String::new();
    let _ = io::stdin().read_line(&mut number);

    println!("Number: {}", number);
}
