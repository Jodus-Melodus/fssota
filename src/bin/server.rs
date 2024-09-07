use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: &str) -> Self {
        Server {
            address: address.to_string(),
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
        let data = self.read(&mut stream)?;

        println!("{}", data);

        self.write(&mut stream, "Hello client")?;

        Ok(())
    }

    fn write(&self, stream: &mut TcpStream, data: &str) -> io::Result<()> {
        stream.write_all(data.as_bytes())
    }

    fn read(&self, stream: &mut TcpStream) -> io::Result<String> {
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer)?;
        let data = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        Ok(data)
    }
}

fn main() -> io::Result<()> {
    let s = Server::new("192.168.0.21:60000");
    s.start()?;

    Ok(())
}
