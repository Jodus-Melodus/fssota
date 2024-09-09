use colored::*;
use local_ip_address::local_ip;
use rand::Rng;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::io::{stdin, stdout, Write};

pub fn get_local_ip() -> Option<String> {
    match local_ip() {
        Ok(ip) => Some(ip.to_string()),
        Err(_) => None
    }
}

pub enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Color {
    Blue(String),
    Green(String),
    Red(String),
    Magenta(String),
}

impl Color {
    pub fn random(value: &str) -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Self::Blue(value.to_string()),
            1 => Self::Green(value.to_string()),
            2 => Self::Red(value.to_string()),
            _ => Self::Magenta(value.to_string()),
        }
    }

    pub fn green(value: &str) -> Self {
        Self::Green(value.to_string())
    }
    pub fn blue(value: &str) -> Self {
        Self::Blue(value.to_string())
    }
    pub fn red(value: &str) -> Self {
        Self::Red(value.to_string())
    }
    pub fn magenta(value: &str) -> Self {
        Self::Magenta(value.to_string())
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Blue(v) => write!(f, "{}", v.blue()),
            Color::Green(v) => write!(f, "{}", v.green()),
            Color::Red(v) => write!(f, "{}", v.red()),
            Color::Magenta(v) => write!(f, "{}", v.magenta()),
        }
    }
}


pub fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Failed to read line");
    buffer.trim().to_string()
}
