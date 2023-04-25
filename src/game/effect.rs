use macroquad::prelude::*;
use std::{ path::Path };
use std::{ time::Duration };

use super::equipmenu::Item;

#[derive(Debug, Clone)]
pub struct Effect {
    texture: Texture2D,
    textures: [Texture2D; 1],
    frame_counter: u32,
    frame_delay: Duration,
    last_frame_update: std::time::Instant,
}

// Frame Size
const FS_SMOKING: Vec2 = Vec2::new(14.0, 14.0);

const SCALE: f32 = 2.0;
const SHUTTER: u64 = 1000;

impl Effect {
    pub async fn new() -> Self {
        // Load Textures
        let smoking_texture = load_texture("assets/effects/smoking.png").await.unwrap();
        smoking_texture.set_filter(FilterMode::Nearest);

        Self {
            texture: smoking_texture,
            textures: [smoking_texture],
            frame_counter: 0,
            frame_delay: Duration::from_millis(SHUTTER),
            last_frame_update: std::time::Instant::now(),
        }
    }

    pub fn update(&mut self, index: usize) {
        let now = std::time::Instant::now();
        let elapsed = now - self.last_frame_update;

        // Get frame limits
        let max_frames = match Item::from_index(index) {
            Some(Item::Empty) => 0,
            Some(Item::Cigs) => 2,
            None => todo!(),
        };

        // Check frame vs time
        if elapsed >= self.frame_delay {
            self.last_frame_update = now;
            let frames = elapsed.as_secs_f32() / self.frame_delay.as_secs_f32();
            self.frame_counter = (self.frame_counter + (frames as u32)) % max_frames;
        }
    }

    pub fn draw(&self, player_bounds: Rect, index: usize) {
        // Set Src

        let frame = (self.frame_counter % 2) as f32;
        let src_rect: Rect = match Item::from_index(index) {
            Some(Item::Empty) => Rect::new(0.0, 0.0, 0.0, 0.0),
            Some(Item::Cigs) => Rect::new(FS_SMOKING.x * frame, 0.0, FS_SMOKING.x, FS_SMOKING.y),
            None => todo!(),
        };

        // Set dest
        let bounds = Rect::new(
            player_bounds.x + player_bounds.w / 2.0 - (src_rect.w * SCALE) / 2.0,
            player_bounds.y - src_rect.h * SCALE,
            src_rect.w * SCALE,
            src_rect.h * SCALE
        );

        draw_texture_ex(self.texture, bounds.x, bounds.y, WHITE, DrawTextureParams {
            source: Some(src_rect),
            dest_size: Some(bounds.size()),
            ..Default::default()
        });
    }
}