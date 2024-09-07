use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use fssota::game::Game;
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
    let mut c = Client::new("192.168.0.21:60000")?;

    c.write("!SCREEN")?;

    let data = c.read()?;
    let game: Game = from_slice(&data)?;
    println!("{}", game);

    c.write("!DISCONNECT")?;

    Ok(())
}
