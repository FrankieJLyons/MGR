use macroquad::prelude::*;

pub struct Map {
    tiles: Vec<Texture2D>,
}

const SCALE: f32 = 2.0;

const MAP_WIDTH: f32 = 512.0 * SCALE;
const MAP_HEIGHT: f32 = 384.0 * SCALE;

impl Map {
    pub async fn new() -> Self {
        // Load all the tile textures and collision maps here
        let mut tiles = Vec::new();

        for i in 0..120 {
            // Check if the tile texture file exists before loading it
            let tile_filename = format!("assets/rooms/main/{:03}.png", i);
            let tile_texture = if let Ok(metadata) = std::fs::metadata(&tile_filename) {
                if metadata.is_file() {
                    load_texture(&tile_filename).await.unwrap()
                } else {
                    Texture2D::empty()
                }
            } else {
                Texture2D::empty()
            };
            tiles.push(tile_texture);

                tile_texture.set_filter(FilterMode::Nearest);
        }

        Map {
            tiles,
        }
    }

    pub fn draw(&self) {
        // Draw all the tiles in the map
        for (i, tile) in self.tiles.iter().enumerate() {
            let tile_texture = tile; // Get the texture associated with the tile
            let x = i as f32 * MAP_WIDTH as f32;
            let y = i as f32 * MAP_HEIGHT as f32;
            let dest_rect = Rect::new(x * MAP_WIDTH, y * MAP_HEIGHT, MAP_WIDTH, MAP_HEIGHT);

            draw_texture_ex(
                *tile_texture,
                dest_rect.x,
                dest_rect.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(dest_rect.size()),
                    ..Default::default()
                },
            );
        }
    }
}