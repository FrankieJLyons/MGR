use macroquad::prelude::*;

use std::fs::File;
use std::io::{ BufRead, BufReader };

use crate::game::Settings;
use crate::game::Room;

pub struct Map {
    settings: Settings,
    pub rooms: Vec<Room>,
}

impl Map {
    pub async fn new(settings: Settings, map_file: &str) -> Self {
        // Load all the room textures and collision maps here
        let mut rooms = Vec::new();

        let mut map_grid = Vec::new();

        // Open the map file and read its contents line by line
        let file = File::open(map_file).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let mut row = Vec::new();

            for room_id in line
                .unwrap()
                .split(",")
                .map(|s| s.trim().to_owned()) {
                row.push(Some(room_id));
            }
            map_grid.push(row);
        }

        // Flatten the map grid into a linear array of rooms
        for (i, row) in map_grid.iter().enumerate() {
            for (j, room) in row.iter().enumerate() {
                if let Some(room_id) = room {
                    let r = Room::new(&room_id, Vec2 { x: j as f32, y: i as f32 }).await;
                    rooms.push(r);
                }
            }
        }

        Map {
            settings,
            rooms,
        }
    }

    pub fn draw(&mut self) {
        self.settings.update();
        // Draw all the rooms in the map grid
        for r in &self.rooms {
            r.draw();
            if self.settings.debug {
                r.draw_debug();
            }
        }
    }
}