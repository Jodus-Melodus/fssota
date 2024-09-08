use crate::{
    objects::{Object, Player, Tile, Tree},
    utils::Direction,
};
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

    pub fn move_player(&mut self, player: &mut Player, direction: Direction) {
        let (mut new_x, mut new_y) = (player.x as isize, player.y as isize);

        match direction {
            Direction::N => {
                new_y -= 1;
            }
            Direction::E => {
                new_x += 1;
            }
            Direction::S => {
                new_y += 1;
            }
            Direction::W => {
                new_x -= 1;
            }
        }

        if (new_x < 64) && (new_y < 64) && (new_x >= 0) && (new_y >= 0) {
            let x = new_x as usize;
            let y = new_y as usize;

            let valid = match &self.map[y][x] {
                Object::Tile(_) => true,
                Object::Tree(_) => false,
                Object::Player(_) => false,
            };

            if valid {
                self.map[player.y][player.x] = Object::Tile(Tile::new());
                (player.x, player.y) = (x, y);
                self.map[y][x] = Object::Player(player.clone());
            }
        }
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
