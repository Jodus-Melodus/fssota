use crate::objects::{Object, Player, Tile, Tree};
use rand::Rng;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    map: Vec<Vec<Object>>,
}

impl Game {
    pub fn new() -> Self {
        let mut map = vec![vec![Object::Tile(Tile::new()); 64]; 64];
        let mut rng = rand::thread_rng();

        for y in 0..64 {
            for x in 0..64 {
                if rng.gen::<f32>() > 0.9 {
                    map[y][x] = Object::Tree(Tree::new())
                }
            }
        }

        Game { map }
    }

    pub fn spawn_player(&mut self, name: &str, symbol: char) -> Player {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..64);
        let y = rng.gen_range(0..64);
        let player = Player::new(name.to_string(), x, y, symbol);
        
        self.map[y][x] = Object::Player(player.clone());
        player
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..64 {
            for x in 0..64 {
                write!(f, "{}", self.map[y][x])?;
            }
            writeln!(f, "")?;
        }
        write!(f, "")
    }
}
