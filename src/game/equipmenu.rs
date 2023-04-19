use macroquad::prelude::*;
use std::{ path::Path };

pub struct EquipMenu {
    pub pause: bool,
    side: bool,
    left_selected: usize,
    left_selected_up: usize,
    left_selected_down: usize,
    right_selected: usize,
    right_selected_up: usize,
    right_selected_down: usize,
    bg_texture: Texture2D,
    item_textures: Vec<Texture2D>,
    weapon_textures: Vec<Texture2D>,
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

const OFFSET_NEXT_X: f32 = 160.0;
const OFFSET_NEXT_Y: f32 = -128.0;

const ITEM_SIZE: usize = 2 - 1;
const WEAPON_SIZE: usize = 2 - 1;

impl EquipMenu {
    pub async fn new() -> Self {
        let bg_texture = load_texture("assets/items/background.png").await.unwrap();
        bg_texture.set_filter(FilterMode::Nearest);

        let mut item_textures = Vec::new();
        let mut weapon_textures = Vec::new();

        for id in 0..=ITEM_SIZE {
            let path = format!("assets/items/{:02}.png", id);

            let p = Path::new(&path);
            if p.exists() {
                let texture = load_texture(&path).await.unwrap();
                texture.set_filter(FilterMode::Nearest);

                item_textures.push(texture);
            }
        }

        for id in 0..=WEAPON_SIZE {
            let path = format!("assets/weapons/{:02}.png", id);

            let p = Path::new(&path);
            if p.exists() {
                let texture = load_texture(&path).await.unwrap();
                texture.set_filter(FilterMode::Nearest);

                weapon_textures.push(texture);
            }
        }

        Self {
            pause: false,
            side: false,
            left_selected: Item::Cigs as usize,
            left_selected_up: Item::Empty as usize,
            left_selected_down: Item::Empty as usize,
            right_selected: Weapon::Handgun as usize,
            right_selected_up: Weapon::Empty as usize,
            right_selected_down: Weapon::Empty as usize,
            bg_texture,
            item_textures,
            weapon_textures,
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

        if self.pause {
            if !self.side {
                if is_key_pressed(KeyCode::Up) {
                    self.left_selected = self.update_index(
                        self.left_selected as f32,
                        1.0,
                        ITEM_SIZE
                    );
                    self.left_selected_up = self.update_index(
                        self.left_selected_up as f32,
                        1.0,
                        ITEM_SIZE
                    );
                    self.left_selected_down = self.update_index(
                        self.left_selected_down as f32,
                        1.0,
                        ITEM_SIZE
                    );
                } else if is_key_pressed(KeyCode::Down) {
                    self.left_selected = self.update_index(
                        self.left_selected as f32,
                        -1.0,
                        ITEM_SIZE
                    );
                    self.left_selected_up = self.update_index(
                        self.left_selected_up as f32,
                        -1.0,
                        ITEM_SIZE
                    );
                    self.left_selected_down = self.update_index(
                        self.left_selected_down as f32,
                        -1.0,
                        ITEM_SIZE
                    );
                }
            } else {
                if is_key_pressed(KeyCode::Up) {
                    self.right_selected = self.update_index(
                        self.right_selected as f32,
                        1.0,
                        WEAPON_SIZE
                    );
                    self.right_selected_up = self.update_index(
                        self.right_selected_up as f32,
                        1.0,
                        WEAPON_SIZE
                    );
                    self.right_selected_down = self.update_index(
                        self.right_selected_down as f32,
                        1.0,
                        WEAPON_SIZE
                    );
                } else if is_key_pressed(KeyCode::Down) {
                    self.right_selected = self.update_index(
                        self.right_selected as f32,
                        -1.0,
                        WEAPON_SIZE
                    );
                    self.right_selected_up = self.update_index(
                        self.right_selected_up as f32,
                        -1.0,
                        WEAPON_SIZE
                    );
                    self.right_selected_down = self.update_index(
                        self.right_selected_down as f32,
                        -1.0,
                        WEAPON_SIZE
                    );
                }
            }
        }
    }

    fn update_index(&self, mut index: f32, direction: f32, limit: usize) -> usize {
        index += direction;
        if index > (limit as f32) {
            index = 0.0;
        } else if index < 0.0 {
            index = limit as f32;
        }
        return index as usize;
    }

    pub fn draw(&self, camera_position: Vec2) {
        if self.pause {
            if !self.side {
                self.draw_left_item(
                    self.left_selected_up,
                    camera_position,
                    Vec2::new(0.0, OFFSET_NEXT_Y)
                );
                self.draw_left_item(self.left_selected, camera_position, Vec2::new(0.0, 0.0));
                self.draw_left_item(
                    self.left_selected_down,
                    camera_position,
                    Vec2::new(OFFSET_NEXT_X, 0.0)
                );
            } else {
                self.draw_right_weapon(
                    self.right_selected_up,
                    camera_position,
                    Vec2::new(0.0, OFFSET_NEXT_Y)
                );
                self.draw_right_weapon(self.right_selected, camera_position, Vec2::new(0.0, 0.0));
                self.draw_right_weapon(
                    self.right_selected_down,
                    camera_position,
                    Vec2::new(-OFFSET_NEXT_X, 0.0)
                );
            }
        } else {
            if self.left_selected != (Item::Empty as usize) {
                self.draw_left_item(self.left_selected, camera_position, Vec2::new(0.0, 0.0));
            }

            if self.right_selected != (Weapon::Empty as usize) {
                self.draw_right_weapon(self.right_selected, camera_position, Vec2::new(0.0, 0.0));
            }
        }
    }

    fn draw_left_item(&self, index: usize, camera_position: Vec2, offset_position: Vec2) {
        draw_texture_ex(
            self.bg_texture,
            camera_position.x - LEFT_X_OS + offset_position.x,
            camera_position.y + Y_OS + offset_position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(WIDTH, HEIGHT)),
                ..Default::default()
            }
        );

        draw_texture_ex(
            self.item_textures[index],
            camera_position.x - LEFT_X_OS + ITEM_X_OFFSET + offset_position.x,
            camera_position.y + Y_OS + ITEM_Y_OFFSET + offset_position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(ITEM_WIDTH, ITEM_HEIGHT)),
                ..Default::default()
            }
        );

        draw_text(
            &Item::from_index(index).unwrap().to_string(),
            camera_position.x - LEFT_X_OS + TEXT_X_OFFSET + offset_position.x,
            camera_position.y + Y_OS + TEXT_Y_OFFSET + offset_position.y,
            24.0,
            WHITE
        );
    }

    fn draw_right_weapon(&self, index: usize, camera_position: Vec2, offset_position: Vec2) {
        draw_texture_ex(
            self.bg_texture,
            camera_position.x + RIGHT_X_0S + offset_position.x,
            camera_position.y + Y_OS + offset_position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(WIDTH, HEIGHT)),
                ..Default::default()
            }
        );

        draw_texture_ex(
            self.weapon_textures[index],
            camera_position.x + RIGHT_X_0S + ITEM_X_OFFSET + offset_position.x,
            camera_position.y + Y_OS + ITEM_Y_OFFSET + offset_position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(ITEM_WIDTH, ITEM_HEIGHT)),
                ..Default::default()
            }
        );

        draw_text(
            &Weapon::from_index(index).unwrap().to_string(),
            camera_position.x + RIGHT_X_0S + TEXT_X_OFFSET + offset_position.x,
            camera_position.y + Y_OS + TEXT_Y_OFFSET + offset_position.y,
            24.0,
            WHITE
        );
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Item {
    Empty = 0,
    Cigs = 1,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Item::Empty => write!(f, "Empty"),
            Item::Cigs => write!(f, "Cigs"),
        }
    }
}

impl Item {
    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Item::Empty),
            1 => Some(Item::Cigs),
            _ => None,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Weapon {
    Empty = 0,
    Handgun = 1,
}

impl std::fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Weapon::Empty => write!(f, "Empty"),
            Weapon::Handgun => write!(f, "Handgun"),
        }
    }
}

impl Weapon {
    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Weapon::Empty),
            1 => Some(Weapon::Handgun),
            _ => None,
        }
    }
}