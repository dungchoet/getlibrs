use std::io;
use std::io::{Write};

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input
}

pub fn get_float_f64(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let input = read_input();

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Format: f64"),
        }
    }
}

pub fn get_int_i32(prompt: &str) -> i32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let input = read_input();

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Format: i32"),            
        }
    }
}

pub fn get_bool(prompt: &str) -> bool {
    loop {
        print!("{} (y/n): ", prompt);
        io::stdout().flush().unwrap();

        let input = read_input();

        match input.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("Usage: y or n then Enter"),
        }
    }
}

pub fn get_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    return read_input().trim().to_string()
}