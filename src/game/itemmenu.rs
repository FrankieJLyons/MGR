use macroquad::prelude::*;
use std::{ path::Path };

pub struct ItemMenu {
    pause: bool,
    side: bool,
    left_selected: Item,
    right_selected: Item,
    bg_texture: Texture2D,
    item_textures: Vec<Texture2D>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Item {
    Empty = 0,
    Handgun = 1,
    Cigs = 2,
}

const WIDTH: f32 = 128.0;
const HEIGHT: f32 = 96.0;
const LEFT_X_OS: f32 = 628.0;
const RIGHT_X_0S: f32 = LEFT_X_OS - WIDTH;
const Y_OS: f32 = 256.0;
const ITEM_WIDTH: f32 = 140.0;
const ITEM_HEIGHT: f32 = 68.0;
const ITEM_X_OFFSET: f32 = WIDTH / 2.0 - ITEM_WIDTH / 2.0;
const ITEM_Y_OFFSET: f32 = HEIGHT / 2.0 - ITEM_HEIGHT / 2.0;
const TEXT_X_OFFSET: f32 = WIDTH / 4.0;
const TEXT_Y_OFFSET: f32 = HEIGHT;

impl ItemMenu {
    pub async fn new() -> Self {
        let bg_texture = load_texture("assets/items/background.png").await.unwrap();
        bg_texture.set_filter(FilterMode::Nearest);

        let mut item_textures = Vec::new();

        for id in 0..=2 {
            let path = format!("assets/items/{:02}.png", id);

            let p = Path::new(&path);
            if p.exists() {
                let texture = load_texture(&path).await.unwrap();
                texture.set_filter(FilterMode::Nearest);

                item_textures.push(texture);
            }
        }

        Self {
            pause: false,
            side: false,
            left_selected: Item::Cigs,
            right_selected: Item::Handgun,
            bg_texture,
            item_textures,
        }
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::Q) {
            self.pause = true;
            self.side = false;
        } else if is_key_down(KeyCode::E) {
            self.pause = true;
            self.side = true;
        } else {
            self.pause = false;
        }
    }

    pub fn draw(&self, camera_position: Vec2) {
        if self.pause {
            if !self.side {
                // Left menu
            } else {
                // Right menu
            }
        }

        if self.left_selected != Item::Empty {
            draw_texture_ex(
                self.bg_texture,
                camera_position.x - LEFT_X_OS,
                camera_position.y + Y_OS,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(WIDTH, HEIGHT)),
                    ..Default::default()
                }
            );

            draw_texture_ex(
                self.item_textures[self.left_selected.clone() as usize],
                camera_position.x - LEFT_X_OS + ITEM_X_OFFSET,
                camera_position.y + Y_OS + ITEM_Y_OFFSET,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(ITEM_WIDTH, ITEM_HEIGHT)),
                    ..Default::default()
                }
            );

            draw_text(
                &self.left_selected.to_string(),
                camera_position.x - LEFT_X_OS + TEXT_X_OFFSET,
                camera_position.y + Y_OS + TEXT_Y_OFFSET,
                24.0,
                WHITE
            );
        }

        if self.right_selected != Item::Empty {
            draw_texture_ex(
                self.bg_texture,
                camera_position.x + RIGHT_X_0S,
                camera_position.y + Y_OS,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(WIDTH, HEIGHT)),
                    ..Default::default()
                }
            );

            draw_texture_ex(
                self.item_textures[self.right_selected.clone() as usize],
                camera_position.x + RIGHT_X_0S + ITEM_X_OFFSET,
                camera_position.y + Y_OS + ITEM_Y_OFFSET,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(ITEM_WIDTH, ITEM_HEIGHT)),
                    ..Default::default()
                }
            );

            draw_text(
                &self.right_selected.to_string(),
                camera_position.x + RIGHT_X_0S + TEXT_X_OFFSET,
                camera_position.y + Y_OS + TEXT_Y_OFFSET,
                24.0,
                WHITE
            );
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Item::Empty => write!(f, "Empty"),
            Item::Handgun => write!(f, "Handgun"),
            Item::Cigs => write!(f, "Cigs"),
        }
    }
}