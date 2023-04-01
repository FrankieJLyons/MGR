use macroquad::prelude::*;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Map {
    tiles: Vec<Texture2D>,
    tile_string: Vec<String>,
}

const SCALE: f32 = 2.0;

const MAP_WIDTH: f32 = 512.0 * SCALE;
const MAP_HEIGHT: f32 = 384.0 * SCALE;

impl Map {
    pub async fn new(map_file: &str) -> Self {
        // Load all the tile textures and collision maps here
        let mut tiles = Vec::new();
        let mut tile_string = Vec::new();

        let mut map_grid = Vec::new();

        // Open the map file and read its contents line by line
        let file = File::open(map_file).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let mut row = Vec::new();
            let line = line.unwrap();
            for tile_id in line.split(",").map(|s| s.trim()) {
                if tile_id == "xxx" {
                    // Insert a default texture instead of None
                    let default_texture = load_texture("assets/rooms/xxx.png").await.unwrap();
                    row.push(Some(default_texture));
                    tile_string.push("".to_string());
                } else {
                    let tile_filename = if tile_id.parse::<i32>().unwrap() <= 120 {
                        format!("assets/rooms/main/{:03}.png", tile_id)
                    } else {
                        format!("assets/rooms/side/{:03}.png", tile_id)
                    };
                    let tile_texture = load_texture(&tile_filename).await.unwrap();
                    tile_texture.set_filter(FilterMode::Nearest);
                    row.push(Some(tile_texture));

                    let tile_str = tile_id.to_string();
                    tile_string.push(tile_str);
                }
            }
            map_grid.push(row);
        }

        // Flatten the map grid into a linear array of tiles
        for row in map_grid {
            for tile in row {
                if let Some(tile_texture) = tile {
                    tiles.push(tile_texture);
                }
            }
        }

        Map {
            tiles,
            tile_string
        }
    }

    pub fn draw(&self) {
        // Draw all the tiles in the map grid
        for (i, tile) in self.tiles.iter().enumerate() {
            let x = (i % 5) as f32 * MAP_WIDTH;
            let y = (i / 5) as f32 * MAP_HEIGHT;
            let dest_rect = Rect::new(x, y, MAP_WIDTH, MAP_HEIGHT);

            draw_texture_ex(
                *tile,
                dest_rect.x,
                dest_rect.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(dest_rect.size()),
                    ..Default::default()
                },
            );

            // Debug info
            draw_text(
                &self.tile_string[i].to_string(),
                dest_rect.x + 16.0,
                dest_rect.y + 32.0,
                64.0,
                WHITE,
            );
        }
    }
}