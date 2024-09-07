use std::{
    io::{self, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

use fssota::game::Game;
use serde_json::to_vec;

pub struct Server {
    address: String,
    game: Game,
}

impl Server {
    pub fn new(address: &str) -> Self {
        Server {
            address: address.to_string(),
            game: Game::new(),
        }
    }

    pub fn start(&self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;

        println!("Server is listening on {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(s) => {
                    self.handle_client(s)?;
                }
                Err(e) => eprintln!("Failed to connect: {}", e),
            }
        }

        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) -> io::Result<()> {
        let mut request;

        loop {
            request = self.read(&mut stream)?;
            println!("{}", request);

            match request.as_str() {
                "!DISCONNECT" => {
                    stream.shutdown(Shutdown::Both)?;
                    break;
                }
                "!SCREEN" => {
                    let bytes = to_vec(&self.game)?;
                    self.write(&mut stream, bytes)?;
                }
                _ => (),
            }
        }

        Ok(())
    }

    fn read(&self, stream: &mut TcpStream) -> io::Result<String> {
        // get the size of the buffer
        let mut size = [0u8; 8];
        stream.read_exact(&mut size)?;
        let length = usize::from_be_bytes(size);

        // receive the actual data
        let mut buffer = vec![0; length];
        stream.read(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

    fn write(&self, stream: &mut TcpStream, bytes: Vec<u8>) -> io::Result<()> {
        let length = bytes.len();
        stream.write_all(&length.to_be_bytes())?;
        stream.write_all(&bytes)
    }
}

fn main() -> io::Result<()> {
    let s = Server::new("192.168.0.21:60000");
    s.start()?;

    Ok(())
}
