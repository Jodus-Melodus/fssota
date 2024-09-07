use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(address: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(address)?;
        Ok(Client { stream })
    }

    pub fn read(&mut self) -> io::Result<String> {
        let mut buffer = [0; 512];
        let bytes_read = self.stream.read(&mut buffer)?;
        let data = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        Ok(data)
    }

    pub fn write(&mut self, data: &str) -> io::Result<()> {
        self.stream.write_all(data.as_bytes())
    }
}

fn main() -> io::Result<()> {
    let mut c = Client::new("192.168.0.21:60000")?;
    c.write("Hello world")?;

    let data = c.read()?;
    println!("{}", data);

    Ok(())
}
