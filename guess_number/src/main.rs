use std::io::{self, Write};

fn main() {

    let rnd_number:u8 = rand::random();
    println!("rnd_number: {}", rnd_number);

    //unwrap_version(rnd_number);

    match_version(rnd_number);
    
}

fn unwrap_version(rnd_number:u8){
    loop{
        print!("Guess number (0-255):");
        let _ = io::stdout().flush();
        
        let mut number_str = String::new();
        let _ = io::stdin().read_line(&mut number_str);

        // Remove extra spaces and new line from Enter. Then parse the string to u8 type.
        // guessed will be a Result<u8> type, no a just <u8> type.
        let guessed = number_str.trim().parse::<u8>();

        //Extract the value from the Result
        let unwrapped = guessed.unwrap(); 

        // The comparisson cannot use directly the unwrap funcion, guessed.unwrap(), 
        // because the unwrap destroy the Result, means the next "if" will fail when try to 
        // do guessed.unwrap() again.
        // The unwrap can be done only once in this code and then reuse the result.
        if  unwrapped > rnd_number {
            println!("Your number {} is higher", number_str.trim());
            continue;
        }

        if unwrapped < rnd_number {
            println!("Your number {} is lower", number_str.trim());
            continue;
        }

        if unwrapped == rnd_number {
            println!("You guess it!");
            break;
        }
    }
}

fn match_version(rnd_number:u8){
    loop{
        print!("Guess number (0-255):");
        let _ = io::stdout().flush();
        
        let mut number_str = String::new();
        let _ = io::stdin().read_line(&mut number_str);

        match number_str.trim().parse::<u8>() {
            Ok(value) => {
                if  value > rnd_number {
                    println!("Your number {} is higher", number_str.trim());
                    continue;
                }

                if value < rnd_number {
                    println!("Your number {} is lower", number_str.trim());
                    continue;
                }

                if value == rnd_number {
                    println!("You guess it!");
                    break;
                }
            },
            Err(err) => {
                println!("Error on conversion: {}", err);
            }
        }
    }
}