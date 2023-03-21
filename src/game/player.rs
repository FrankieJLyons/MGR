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
        let frame_width = self.texture.width() / 4.0 + 4.0; // Assuming 4 frames in the sprite sheet
        let frame_height = self.texture.height() / 4.0 + 2.0;

        let scale = Vec2::new(4.0, 4.0);
        let dest_rect = Rect::new(
            (screen_width() - frame_width * scale.x) / 2.0,
            (screen_height() - frame_height * scale.y) / 2.0,
            frame_width * scale.x,
            frame_height * scale.y,
        );

        let src_rect = Rect::new(0.0, 0.0, frame_width, frame_height);

        self.texture.set_filter(FilterMode::Nearest);

        draw_texture_ex(
            self.texture,
            dest_rect.x,
            dest_rect.y,
            WHITE,
            DrawTextureParams {
                source: Some(src_rect),
                dest_size: Some(dest_rect.size()),
                ..Default::default()
            },
        );
    }
}