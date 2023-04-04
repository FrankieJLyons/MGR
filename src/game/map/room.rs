use macroquad::prelude::*;
use std::path::Path;

mod colliderMap;

use colliderMap::ColliderMap;

pub struct Room {
    name: String,
    texture: Texture2D,
    position: Vec2,
    bounds: Rect,
    collider_map: ColliderMap
}

const MAP_SCALE: f32 = 2.0;
const MAP_WIDTH: f32 = 512.0 * MAP_SCALE;
const MAP_HEIGHT: f32 = 384.0 * MAP_SCALE;

impl Room {
    pub async fn new(id: &str, pos_id: Vec2) -> Self {
        let texture_path: String;
        if id == "xxx" {
            texture_path = "assets/rooms/xxx.png".to_string();
        } else {
            if id.parse::<i32>().unwrap() <= 120 {
                texture_path = format!("assets/rooms/main/{:03}.png", id);
            } else {
                texture_path = format!("assets/rooms/side/{:03}.png", id);
            };
        }
        let texture = load_texture(&texture_path).await.unwrap();
        texture.set_filter(FilterMode::Nearest);

        let position = Vec2 { x: (pos_id.x % 5.0) * MAP_WIDTH, y: pos_id.y * MAP_HEIGHT};
        let bounds = Rect::new(position.x, position.y, MAP_WIDTH, MAP_HEIGHT);

        let collider_path = format!("assets/rooms/colliders/{:03}.png", id);
        let collider_map: ColliderMap;

        let p = Path::new(&collider_path);
        if p.exists() {
            collider_map = ColliderMap::new(&collider_path).await;
        } else {
            // eprintln!("File does not exist: {}", &collider_path);
            collider_map = ColliderMap::new("assets/rooms/colliders/xxx.png").await;
        }

        Room {
            name: id.to_string(),
            texture,
            position,
            bounds,
            collider_map
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.bounds.x,
            self.bounds.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.bounds.size()),
                ..Default::default()
            },
        );

        // Debug info
        self.collider_map.draw(self.bounds.x, self.bounds.y);

        draw_text(
            &self.name,
            self.position.x + 16.0,
            self.position.y + 32.0,
            64.0,
            WHITE,
        );
    }
}