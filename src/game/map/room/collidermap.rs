use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct ColliderMap {
    pub name: String,
    texture: Texture2D,
    pub colliders: Vec<Rect>,
    parent_bounds: Rect
}

const SIZE: f32 = 32.0;

impl ColliderMap {
    pub async fn new(path: &str, parent_bounds: Rect) -> Self {
        let image = load_image(path).await.unwrap();
        let texture = load_texture(path).await.unwrap();

        let mut colliders = Vec::new();
        for y in 0..image.height() {
            for x in 0..image.width() {
                let pixel = image.get_pixel(x as u32, y as u32);

                let n_pix = image.get_pixel(x as u32, (y - 1) as u32);

                if pixel == BLACK {
                    let collider_rect = Rect::new(
                        (x as f32) * SIZE + parent_bounds.x,
                        (y as f32) * SIZE + parent_bounds.y,
                        SIZE,
                        SIZE,
                    );
                    colliders.push(collider_rect);
                }
            }
        }

        ColliderMap {
            name: path.to_string(),
            texture,
            colliders,
            parent_bounds
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.parent_bounds.x,
            self.parent_bounds.y,
            Color::new(1.0, 1.0, 1.0, 0.5),
            DrawTextureParams {
                dest_size: Some(self.parent_bounds.size()),
                ..Default::default()
            },
        );

        for collider in &self.colliders {
            draw_rectangle(
                collider.x,
                collider.y,
                collider.w,
                collider.h,
                Color::new(1.0, 0.0, 0.0, 0.5),
            );
        }
    }
}