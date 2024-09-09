use crate::{
    objects::{Object, Player, Tile, Tree, Water},
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
        Self::populate_trees(width, height, &mut map);
        Self::populate_water(width, height, &mut map);

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
                Object::Water(_) => false,
            };

            if valid {
                self.map[player.y][player.x] = Object::Tile(Tile::new());
                (player.x, player.y) = (x, y);
                self.map[y][x] = Object::Player(player.clone());
            }
        }
    }

    pub fn game_from_player_view(&self, player: Player) -> Game {
        let x_view_range = (player.x.saturating_sub(40))..(player.x + 40);
        let y_view_range = (player.y.saturating_sub(20))..(player.y + 20);

        let mut view_map =
            vec![
                vec![Object::Tile(Tile::new()); (x_view_range.end - x_view_range.start) as usize];
                (y_view_range.end - y_view_range.start) as usize
            ];

        for (y, row) in y_view_range.enumerate() {
            for (x, col) in x_view_range.clone().enumerate() {
                if let Some(ref game_row) = self.map.get(row) {
                    if let Some(&ref object) = game_row.get(col) {
                        view_map[y][x] = object.clone();
                    }
                }
            }
        }

        Game {
            map: view_map.clone(),
            width: view_map[0].len(),
            height: view_map.len(),
        }
    }

    fn populate_water(width: usize, height: usize, map: &mut Vec<Vec<Object>>) {
        let mut rng = rand::thread_rng();
        let adjacent_coordinates =
            [(1, 1), (1, -1), (-1, -1), (-1, 1), (0, 1), (0, -1), (1, 0), (-1, 0)];

        let water_density = ((width * height) as f32 * 0.0005) as usize;

        for _ in 0..water_density {
            let (x, y) = (rng.gen_range(2..width - 2), rng.gen_range(2..height - 2));
            map[y][x] = Object::Water(Water::new());
            Self::populate_adjacent_water(x, y, &adjacent_coordinates, map, &mut rng, 0.7, 5);
        }
    }

    fn populate_adjacent_water(
        x: usize,
        y: usize,
        adjacent_coordinates: &[(i8, i8)],
        map: &mut Vec<Vec<Object>>,
        rng: &mut rand::rngs::ThreadRng,
        probability: f64,
        depth: u8,
    ) {
        if depth > 0 {
            for &(dx, dy) in adjacent_coordinates {
                let (new_x, new_y) = (x as i8 + dx, y as i8 + dy);
                if new_x >= 0 && new_y >= 0 && new_y < map.len() as i8 && new_x < map[0].len() as i8
                {
                    let (new_x, new_y) = (new_x as usize, new_y as usize);
                    if rng.gen_bool(probability) {
                        map[new_y][new_x] = Object::Water(Water::new());
                        Self::populate_adjacent_water(
                            new_x,
                            new_y,
                            adjacent_coordinates,
                            map,
                            rng,
                            probability - 0.05,
                            depth - 1,
                        );
                    }
                }
            }
        }
    }

    fn populate_trees(width: usize, height: usize, map: &mut Vec<Vec<Object>>) {
        let mut rng = rand::thread_rng();
        let adjacent_coordinates: [(i8, i8); 8] = [
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, -1),
            (-1, 0),
            (0, 1),
            (1, 0),
        ];

        let tree_density = ((width * height) as f32 * 0.03) as usize;

        for _ in 0..tree_density {
            let (x, y) = (rng.gen_range(1..width - 1), rng.gen_range(1..height - 1));
            map[y][x] = Object::Tree(Tree::new());

            for adjacent_coordinate in adjacent_coordinates {
                if rng.gen_bool(0.5) {
                    map[(y as i8 + adjacent_coordinate.1) as usize]
                        [(x as i8 + adjacent_coordinate.0) as usize] = Object::Tree(Tree::new());
                }
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
