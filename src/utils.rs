use std::io::{stdin, stdout, Write};

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
