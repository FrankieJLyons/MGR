use macroquad::prelude::*;

pub struct Player {
    texture: Texture2D,
}

impl Player {
    pub async fn new() -> Self {
        let texture = load_texture("assets/snake/snake_walking.png").await.unwrap();
        Self { texture }
    }

    pub fn update(&mut self) {}

    pub fn draw(&self) {
        draw_texture(self.texture, 0.0, 0.0, WHITE);
    }
}