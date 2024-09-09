use std::{
    io::{self, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use fssota::{
    chat::{Chat, Message},
    game::Game,
    utils::{get_local_ip, Direction},
};
use serde_json::to_vec;

pub struct Server {
    address: String,
    game: Game,
    chat: Chat,
}

impl Server {
    pub fn new(address: &str) -> Self {
        Server {
            address: address.to_string(),
            game: Game::new(100, 100),
            chat: Chat::new(),
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
        let symbol = Self::read(&mut stream)?.chars().next().unwrap();

        println!("{} joined as {}", name, symbol);

        let mut player;
        {
            player = server.lock().unwrap().game.spawn_player(&name, symbol);
        }

        let mut request;
        loop {
            request = Self::read(&mut stream)?;

            match request.as_str() {
                "!DISCONNECT" => {
                    println!("{} disconnected", name);
                    let game = &mut server.lock().unwrap().game;
                    game.kill_player(player);
                    stream.shutdown(Shutdown::Both)?;
                    break;
                }
                "!SCREEN" => {
                    let game = &server.lock().unwrap().game.game_from_player_view(player.clone());
                    let bytes = to_vec(&game)?;
                    Self::write(&mut stream, bytes)?;
                    let bytes = to_vec(&player.inventory)?;
                    Self::write(&mut stream, bytes)?;
                }
                "!MOVE" => {
                    let direction = Self::read(&mut stream)?;
                    let direction = match direction.chars().next().unwrap() {
                        'w' => Direction::N,
                        'd' => Direction::E,
                        's' => Direction::S,
                        'a' => Direction::W,
                        _ => panic!(),
                    };

                    let game = &mut server.lock().unwrap().game;
                    game.move_player(&mut player, direction);
                }
                "!CHAT" => {
                    let chat = &server.lock().unwrap().chat;
                    let bytes = to_vec(&chat)?;
                    Self::write(&mut stream, bytes)?;
                }
                "!NEWMESSAGE" => {
                    let msg = Self::read(&mut stream)?;
                    let message = Message::new(player.clone(), &msg);
                    let chat = &mut server.lock().unwrap().chat;
                    chat.add_new_message(message);

                    println!("{} said: {}", player.name, msg);
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
    if let Some(ip) = get_local_ip() {
        let port = "60000";
        let address = format!("{}:{}", ip, port);
        let server = Arc::new(Mutex::new(Server::new(&address)));
        Server::start(server)?;
    }
    Ok(())
}
