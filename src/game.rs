use crate::{
    objects::{Object, Player, Tile, Tree},
    utils::Direction,
};
use rand::Rng;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    width: usize,
    height: usize,
    map: Vec<Vec<Object>>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let mut map = vec![vec![Object::Tile(Tile::new()); width]; height];
        let mut rng = rand::thread_rng();

        for y in 0..height {
            for x in 0..width {
                if rng.gen::<f32>() > 0.9 {
                    map[y][x] = Object::Tree(Tree::new())
                }
            }
        }

        Game { width, height, map }
    }

    pub fn spawn_player(&mut self, name: &str, symbol: char) -> Player {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..self.width);
        let y = rng.gen_range(0..self.height);
        let player = Player::new(name.to_string(), x, y, symbol);

        self.map[y][x] = Object::Player(player.clone());
        player
    }

    pub fn kill_player(&mut self, player: Player) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.map[y][x] == Object::Player(player.clone()) {
                    self.map[y][x] = Object::Tile(Tile::new());
                    return;
                }
            }
        }
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

        if (new_x < self.width as isize)
            && (new_y < self.height as isize)
            && (new_x >= 0)
            && (new_y >= 0)
        {
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
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.map[y][x])?;
            }
            writeln!(f, "")?;
        }
        write!(f, "")
    }
}
