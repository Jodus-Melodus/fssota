use objects::Tile;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Object {
    Tile(Tile),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tile(t) => write!(f, "{}", t),
        }
    }
}

pub mod objects {
    use std::fmt;
    use serde_derive::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Tile {}

    impl Tile {
        pub fn new() -> Self {
            Tile {}
        }
    }

    impl fmt::Display for Tile {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} ", ".")
        }
    }

    impl Clone for Tile {
        fn clone(&self) -> Self {
            Tile {}
        }
    }
}
