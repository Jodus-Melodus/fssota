use colored::Colorize;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Object {
    Tile(Tile),
    Tree(Tree),
    Player(Player),
    Water(Water)
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tile(t) => write!(f, "{}", t),
            Self::Tree(t) => write!(f, "{}", t),
            Self::Player(p) => write!(f, "{}", p),
            Self::Water(w) => write!(f, "{}", w), 
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Water {}

impl Water {
    pub fn new() -> Self {
        Water {}
    }
}

impl fmt::Display for Water {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", "~".blue())
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
        write!(f, "{} ", "▲".green())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub name: String,
    pub x: usize,
    pub y: usize,
    pub inventory: Vec<Object>,
    symbol: char,
}

impl Player {
    pub fn new(name: String, x: usize, y: usize, symbol: char) -> Self {
        Player { name, x, y, symbol, inventory: Vec::new() }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.symbol.to_string().red())
    }
}