use serde_derive::{Deserialize, Serialize};
use std::fmt;

use crate::utils::Color;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Object {
    Tile(Tile),
    Tree(Tree),
    Player(Player),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tile(t) => write!(f, "{}", t),
            Self::Tree(t) => write!(f, "{}", t),
            Self::Player(p) => write!(f, "{}", p),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Tile {}

impl Tile {
    pub fn new() -> Self {
        Tile {}
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  ")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Tree {}

impl Tree {
    pub fn new() -> Self {
        Tree {}
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", Color::green("▲"))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub name: String,
    pub x: usize,
    pub y: usize,
    symbol: char,
    color: Color,
}

impl Player {
    pub fn new(name: String, x: usize, y: usize, symbol: char) -> Self {
        Player {
            name,
            x,
            y,
            symbol,
            color: Color::random(&symbol.to_string()),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.color)
    }
}
