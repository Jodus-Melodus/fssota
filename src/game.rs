use serde_derive::{Deserialize, Serialize};

use crate::objects::{Object, objects::Tile};
use std::fmt::{self};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    map: Vec<Vec<Object>>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            map: vec![vec![Object::Tile(Tile::new()); 64]; 64],
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
