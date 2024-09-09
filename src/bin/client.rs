use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use fssota::{
    chat::Chat,
    game::Game,
    utils::{clear_terminal, read_line},
};
use serde_json::from_slice;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(address: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(address)?;
        Ok(Client { stream })
    }

    pub fn handle(&mut self) -> io::Result<()> {
        let mut name = read_line("Enter your name (leave blank to use username) > ");
        if name.is_empty() {
            name = whoami::username();
        }
        self.write(&name)?;

        let mut symbol = read_line("Enter your symbol (leave blank for default) > ");
        if symbol.is_empty() {
            symbol = "@".to_string();
        }
        self.write(&symbol)?;

        loop {
            if event::poll(std::time::Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    if event.kind == KeyEventKind::Press {
                        match event.code {
                            KeyCode::Char(c) => match c {
                                'w' => {
                                    self.write("!MOVE")?;
                                    self.write("w")?;
                                }
                                'a' => {
                                    self.write("!MOVE")?;
                                    self.write("a")?;
                                }
                                's' => {
                                    self.write("!MOVE")?;
                                    self.write("s")?;
                                }
                                'd' => {
                                    self.write("!MOVE")?;
                                    self.write("d")?;
                                }
                                '/' => {
                                    self.write("!CHAT")?;
                                    let bytes = self.read()?;
                                    let chat: Chat = from_slice(&bytes)?;
                                    println!("{}", chat);
                                    let message = read_line("> ");

                                    if message.len() > 0 {
                                        self.write("!NEWMESSAGE")?;
                                        self.write(&message)?;
                                    }
                                }
                                _ => {}
                            },
                            KeyCode::Esc => {
                                self.write("!DISCONNECT")?;
                                break;
                            }
                            _ => {}
                        }
                        self.write("!SCREEN")?;
                        let bytes = self.read()?;
                        let game: Game = from_slice(&bytes)?;
                        clear_terminal();
                        println!("{}", game);
                    }
                }
            }
        }
        Ok(())
    }

    fn read(&mut self) -> io::Result<Vec<u8>> {
        // get the size of the buffer
        let mut size = [0u8; 8];
        self.stream.read_exact(&mut size)?;
        let length = usize::from_be_bytes(size);

        // receive the actual data
        let mut buffer = vec![0; length];
        self.stream.read(&mut buffer)?;
        Ok(buffer)
    }

    fn write(&mut self, data: &str) -> io::Result<()> {
        let bytes = data.as_bytes();
        let length = bytes.len();
        self.stream.write_all(&length.to_be_bytes())?;
        self.stream.write_all(bytes)
    }
}

fn main() -> io::Result<()> {
    println!(
        r"
   ____     ____           ______    _         _   ____              _                          ___  __  __         ___                      __                 
  / __/__ _/ / /__ ___    / __/ /__ (_)__ ___ (_) / __/_ _______  __(_)  _____  _______   ___  / _/ / /_/ /  ___   / _ | ___  ___  _______ _/ /_ _____  ___ ___ 
 / _// _ `/ / / -_) _ \  _\ \/  '_// / -_|_-<_   _\ \/ // / __/ |/ / / |/ / _ \/ __(_-<  / _ \/ _/ / __/ _ \/ -_) / __ |/ _ \/ _ \/ __/ _ `/ / // / _ \(_-</ -_)
/_/  \_,_/_/_/\__/_//_/ /___/_/\_\/_/\__/___(_) /___/\_,_/_/  |___/_/|___/\___/_/ /___/  \___/_/   \__/_//_/\__/ /_/ |_/ .__/\___/\__/\_,_/_/\_, / .__/___/\__/ 
                                                                                                                      /_/                   /___/_/             "
    );

    let server_ip_address = read_line("Enter server IP address > ");
    let port_number = "60000";
    let address = format!("{}:{}", server_ip_address, port_number);
    let mut connect_attempts = 5;

    let mut client: Result<Client, _> = Err("Failed to connect");

    while connect_attempts > 0 {
        match Client::new(&address) {
            Ok(c) => {
                client = Ok(c);
                println!("Successfully connected to {}", address);
                break;
            }
            Err(e) => {
                connect_attempts -= 1;
                eprintln!(
                    "Failed to connect. {} attemps left. Error: {}",
                    connect_attempts, e
                );
            }
        }
    }

    match client {
        Ok(mut c) => c.handle()?,
        Err(_) => eprintln!("Could not connect to the server after multiple attemps"),
    }

    Ok(())
}
