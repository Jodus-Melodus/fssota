use local_ip_address::local_ip;
use std::io::{stdin, stdout, Write};

pub fn get_local_ip() -> Option<String> {
    match local_ip() {
        Ok(ip) => Some(ip.to_string()),
        Err(_) => None,
    }
}

use std::process::Command;

pub fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

pub enum Direction {
    N,
    E,
    S,
    W,
}

pub fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Failed to read line");
    buffer.trim().to_string()
}
