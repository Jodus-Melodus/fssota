use std::{
    io::{self, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
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

    pub fn start(server: Arc<Mutex<Self>>) -> io::Result<()> {
        let listener = TcpListener::bind(&server.lock().unwrap().address)?;

        println!("Server is listening on {}", server.lock().unwrap().address);

        for stream in listener.incoming() {
            let server = Arc::clone(&server);
            match stream {
                Ok(s) => {
                    thread::spawn(move || Self::handle_client(server, s));
                }
                Err(e) => eprintln!("Failed to connect: {}", e),
            }
        }

        Ok(())
    }

    fn handle_client(server: Arc<Mutex<Server>>, mut stream: TcpStream) -> io::Result<()> {
        let name = Self::read(&mut stream)?;
        
        println!("{} joined!", name);

        let symbol = Self::read(&mut stream)?.chars().next().unwrap();

        let player;
        {
            player = server.lock().unwrap().game.spawn_player(&name, symbol);
        }

        let mut request;
        loop {
            request = Self::read(&mut stream)?;

            match request.as_str() {
                "!DISCONNECT" => {
                    println!("{} disconnected", name);
                    stream.shutdown(Shutdown::Both)?;
                    break;
                }
                "!SCREEN" => {
                    let game = &server.lock().unwrap().game;
                    let bytes = to_vec(&game)?;
                    Self::write(&mut stream, bytes)?;
                }
                _ => println!("Requested: {}", request),
            }
        }

        Ok(())
    }

    fn read(stream: &mut TcpStream) -> io::Result<String> {
        let mut size = [0u8; 8];
        stream.read_exact(&mut size)?;
        let length = usize::from_be_bytes(size);

        let mut buffer = vec![0; length];
        stream.read_exact(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

    fn write(stream: &mut TcpStream, bytes: Vec<u8>) -> io::Result<()> {
        let length = bytes.len();
        stream.write_all(&length.to_be_bytes())?;
        stream.write_all(&bytes)
    }
}

fn main() -> io::Result<()> {
    let server = Arc::new(Mutex::new(Server::new("192.168.0.21:60000")));
    Server::start(server)?;

    Ok(())
}
