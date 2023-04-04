use macroquad::prelude::*;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod room;
use self::room::Room;

pub struct Map {
    pub rooms: Vec<Room>,
}

impl Map {
    pub async fn new(map_file: &str) -> Self {
        // Load all the tile textures and collision maps here
        let mut tiles = Vec::new();

        let mut map_grid = Vec::new();

        // Open the map file and read its contents line by line
        let file = File::open(map_file).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let mut row = Vec::new();

            for tile_id in line.unwrap().split(",").map(|s| s.trim().to_owned()) {
                row.push(Some(tile_id));
            }
            map_grid.push(row);
        }

        // Flatten the map grid into a linear array of tiles
        for (i, row) in map_grid.iter().enumerate() {
            for (j, room) in row.iter().enumerate()  {
                if let Some(tile_id) = room {
                    let r = Room::new(&tile_id, Vec2 { x: j as f32, y: i as f32}).await;
                    tiles.push(r);
                }
            }
        }

        Map {
            rooms: tiles
        }
    }

    pub fn draw(&self) {
        // Draw all the tiles in the map grid
        for r in &self.rooms {
            r.draw();
        }
    }
}