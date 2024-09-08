use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use fssota::{chat::Chat, game::Game, utils::read_line};
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

        let name = read_line("Enter your name > ");
        self.write(&name)?;

        let symbol = read_line("Enter your symbol > ");
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
                                },
                                'a' => {
                                    self.write("!MOVE")?;
                                    self.write("a")?;
                                },
                                's' => {
                                    self.write("!MOVE")?;
                                    self.write("s")?;
                                },
                                'd' => {
                                    self.write("!MOVE")?;
                                    self.write("d")?;
                                },
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
    let address = "192.168.0.21:60000";
    println!("Connecting to {}", address);
    let mut c = Client::new(&address).unwrap();
    println!("Starting game loop");
    c.handle()?;

    Ok(())
}
