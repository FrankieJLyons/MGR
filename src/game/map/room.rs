use macroquad::prelude::*;

pub struct Room {
    name: String,
    texture: Texture2D,
    // collider: Vec<Vec<bool>>,
    position: Vec2,
    bounds: Rect
}

const SCALE: f32 = 2.0;

const MAP_WIDTH: f32 = 512.0 * SCALE;
const MAP_HEIGHT: f32 = 384.0 * SCALE;

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

        // let collider_path = format!("assets/rooms/main/{:03}_collider.png", id);
        // let collider_image = load_image(&collider_path).await.unwrap();

        // let mut collider = Vec::new();
        // for y in 0..collider_image.height() {
        //     let mut row = Vec::new();
        //     for x in 0..collider_image.width() {
        //         let pixel = collider_image.get_pixel(x, y);
        //         let is_collidable = pixel.0[0] == 0;
        //         row.push(is_collidable);
        //     }
        //     collider.push(row);
        // }

        Room {
            name: id.to_string(),
            texture,
            // collider,
            position,
            bounds,
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
        draw_text(
            &self.name,
            self.position.x + 16.0,
            self.position.y + 32.0,
            64.0,
            WHITE,
        );
    }

    // pub fn is_collidable(&self, x: f32, y: f32) -> bool {
    //     let tile_x = (x / self.texture.width()) as usize;
    //     let tile_y = (y / self.texture.height()) as usize;
    //     if let Some(row) = self.collider.get(tile_y) {
    //         if let Some(is_collidable) = row.get(tile_x) {
    //             return *is_collidable;
    //         }
    //     }
    //     false
    // }
}