use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Bullet {
    texture: Texture2D,
    alive: bool,
    position: Vec2,
    direction: Vec2,
    size: Vec2,
    speed: f32,
}

const SCALE: f32 = 2.0;
const SPEED: f32 = 512.0;

impl Bullet {
    pub async fn new(position: Vec2, direction: Vec2, size: Vec2) -> Self {
        // Load Textures
        let texture = load_texture("assets/effects/bullet.png").await.unwrap();
        texture.set_filter(FilterMode::Nearest);

        Self {
            texture,
            alive: true,
            position,
            direction,
            size,
            speed: SPEED,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.alive {
            self.position.x += self.direction.x * self.speed * delta_time;
            self.position.y += self.direction.y * self.speed * delta_time;

            eprintln!("Position: {:?}", self.position);
        }
    }

    pub fn draw(&self) {
        if self.alive {
            let src_rect = Rect::new(0.0, 0.0, 4.0, 4.0);

            // Set dest
            let bounds = Rect::new(self.position.x, self.position.y, src_rect.w, src_rect.h);

            draw_texture_ex(self.texture, bounds.x, bounds.y, WHITE, DrawTextureParams {
                source: Some(src_rect),
                dest_size: Some(bounds.size()),
                ..Default::default()
            });
        }
    }
}