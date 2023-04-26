use macroquad::prelude::*;
use std::{ time::Duration };

use crate::game::Settings;
use crate::game::EquipMenu;
use crate::game::Effect;
use crate::game::Bullet;

use super::effect;
use super::equipmenu::Item;
use super::equipmenu::Weapon;

#[derive(Debug, Clone)]
pub struct Player {
    settings: Settings,
    pub equip_menu: EquipMenu,
    effect: Effect,
    texture: Texture2D,
    textures: [Texture2D; 4],
    frame_counter: u32,
    frame_delay: Duration,
    last_frame_update: std::time::Instant,
    last_effect_update: std::time::Instant,
    state: State,
    pub direction: Direction,
    pub position: Vec2,
    pub speed: f32,
    pub bounds: Rect,
    pub collider: Rect,
    pub col_arr: [bool; 4],
    pub health: f32,
    pub bullets: Vec<Bullet>,
}

// Enums
#[derive(Debug, Clone, PartialEq)]
enum State {
    Standing,
    StandingGun,
    Walking,
    WalkingGun,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Frame Size
const FS_STANDING: Vec2 = Vec2::new(17.0, 30.0);
const FS_STANDING_GUN: Vec2 = Vec2::new(17.0, 30.0);
const FS_WALKING: Vec2 = Vec2::new(17.0, 30.0);
const FS_WALKING_GUN: Vec2 = Vec2::new(18.0, 31.0);

// Max Frames
const MF_WALKING: u32 = 2;
const MF_WALKING_GUN: u32 = 2;

// Conts
const START_POS: Vec2 = Vec2::new(512.0 - FS_STANDING.x / 2.0, 384.0 * 8.5);
const SCALE: f32 = 3.0;
const SPEED: f32 = 256.0;
const SHUTTER: u64 = 224;

// const DEBUG_POS: Vec2 = Vec2::new(512.0 - FS_STANDING.x / 2.0 + 512.0 * 8.0, 384.0 * 4.5);

impl Player {
    // Public
    pub async fn new(settings: Settings) -> Self {
        let equip_menu = EquipMenu::new().await;
        let effect = Effect::new().await;

        // Load Textures
        let standing_texture = load_texture("assets/snake/standing.png").await.unwrap();
        standing_texture.set_filter(FilterMode::Nearest);

        let walking_texture = load_texture("assets/snake/walking.png").await.unwrap();
        walking_texture.set_filter(FilterMode::Nearest);

        let standing_gun_texture = load_texture("assets/snake/standing_gun.png").await.unwrap();
        standing_gun_texture.set_filter(FilterMode::Nearest);

        let walking_gun_texture = load_texture("assets/snake/walking_gun.png").await.unwrap();
        walking_gun_texture.set_filter(FilterMode::Nearest);

        // Pattern: Up, Down, Left, Right
        let col_arr = [false, false, false, false];

        let mut bullets = Vec::new();

        // Set self
        Self {
            settings,
            equip_menu,
            effect,
            texture: standing_texture,
            textures: [
                standing_texture,
                walking_texture,
                standing_gun_texture,
                walking_gun_texture,
            ],
            state: State::StandingGun,
            direction: Direction::Up,
            position: START_POS,
            speed: SPEED,
            frame_counter: 0,
            frame_delay: Duration::from_millis(SHUTTER),
            last_frame_update: std::time::Instant::now(),
            last_effect_update: std::time::Instant::now(),
            bounds: Rect::new(
                START_POS.x,
                START_POS.y,
                FS_STANDING.x * SCALE,
                FS_STANDING.y * SCALE
            ),
            collider: Rect::new(
                START_POS.x,
                START_POS.y + FS_STANDING.y * SCALE * 0.4,
                FS_STANDING.x,
                FS_STANDING.y * SCALE * 0.6
            ),
            col_arr,
            health: 100.0,
            bullets,
        }
    }

    pub async fn update(&mut self, delta_time: f32) {
        // Pattern:
        //// State,
        //// Texture,
        //// Input,
        //// Equipment,
        //// Other,

        match self.state {
            State::Standing => {
                if self.texture != self.textures[0] {
                    self.texture = self.textures[0];
                }

                if
                    is_key_down(KeyCode::Up) ||
                    is_key_down(KeyCode::Down) ||
                    is_key_down(KeyCode::Left) ||
                    is_key_down(KeyCode::Right)
                {
                    self.state = State::Walking;
                    self.frame_counter = 0;

                    self.col_arr = [false, false, false, false];
                }
            }

            State::StandingGun => {
                if self.texture != self.textures[2] {
                    self.texture = self.textures[2];
                }

                if
                    is_key_down(KeyCode::Up) ||
                    is_key_down(KeyCode::Down) ||
                    is_key_down(KeyCode::Left) ||
                    is_key_down(KeyCode::Right)
                {
                    self.state = State::WalkingGun;
                    self.frame_counter = 0;

                    self.col_arr = [false, false, false, false];
                }

                if is_key_pressed(KeyCode::F) || is_mouse_button_pressed(MouseButton::Left) {
                    self.fire_weapon().await;
                }
            }

            State::Walking => {
                if self.texture != self.textures[1] {
                    self.texture = self.textures[1];
                }

                if
                    !is_key_down(KeyCode::Up) &&
                    !is_key_down(KeyCode::Down) &&
                    !is_key_down(KeyCode::Left) &&
                    !is_key_down(KeyCode::Right)
                {
                    self.state = State::Standing;
                    self.frame_counter = 0;
                } else {
                    self.update_frame_counter();
                    if is_key_down(KeyCode::Up) {
                        self.direction = Direction::Up;
                        if !self.col_arr[0] {
                            self.position.y -= self.speed * delta_time;
                        }
                        self.col_arr = [self.col_arr[0], false, false, false];
                    } else if is_key_down(KeyCode::Down) {
                        self.direction = Direction::Down;
                        if !self.col_arr[1] {
                            self.position.y += self.speed * delta_time;
                        }
                        self.col_arr = [false, self.col_arr[1], false, false];
                    } else if is_key_down(KeyCode::Left) {
                        self.direction = Direction::Left;
                        if !self.col_arr[2] {
                            self.position.x -= self.speed * delta_time;
                        }
                        self.col_arr = [false, false, self.col_arr[2], false];
                    } else if is_key_down(KeyCode::Right) {
                        self.direction = Direction::Right;
                        if !self.col_arr[3] {
                            self.position.x += self.speed * delta_time;
                        }
                        self.col_arr = [false, false, false, self.col_arr[3]];
                    }
                }
            }

            State::WalkingGun => {
                if self.texture != self.textures[3] {
                    self.texture = self.textures[3];
                }

                if
                    !is_key_down(KeyCode::Up) &&
                    !is_key_down(KeyCode::Down) &&
                    !is_key_down(KeyCode::Left) &&
                    !is_key_down(KeyCode::Right)
                {
                    self.state = State::StandingGun;
                    self.frame_counter = 0;
                } else {
                    self.update_frame_counter();
                    if is_key_down(KeyCode::Up) {
                        self.direction = Direction::Up;
                        if !self.col_arr[0] {
                            self.position.y -= self.speed * delta_time;
                        }
                        self.col_arr = [self.col_arr[0], false, false, false];
                    } else if is_key_down(KeyCode::Down) {
                        self.direction = Direction::Down;
                        if !self.col_arr[1] {
                            self.position.y += self.speed * delta_time;
                        }
                        self.col_arr = [false, self.col_arr[1], false, false];
                    } else if is_key_down(KeyCode::Left) {
                        self.direction = Direction::Left;
                        if !self.col_arr[2] {
                            self.position.x -= self.speed * delta_time;
                        }
                        self.col_arr = [false, false, self.col_arr[2], false];
                    } else if is_key_down(KeyCode::Right) {
                        self.direction = Direction::Right;
                        if !self.col_arr[3] {
                            self.position.x += self.speed * delta_time;
                        }
                        self.col_arr = [false, false, false, self.col_arr[3]];
                    }
                }
            }
        }

        if self.equip_menu.right_selected > 0 {
            if self.state == State::Standing {
                self.state = State::StandingGun;
            } else if self.state == State::Walking {
                self.state = State::WalkingGun;
            }
        } else {
            if self.state == State::StandingGun {
                self.state = State::Standing;
            } else if self.state == State::WalkingGun {
                self.state = State::Walking;
            }
        }

        for bullet in self.bullets.iter_mut() {
            bullet.update(delta_time);
        }

        self.settings.update();
        if self.settings.debug {
            self.speed = SPEED * 2.0;
        } else {
            self.speed = SPEED;
        }
    }

    pub fn update_equipment(&mut self) {
        self.equip_menu.update();

        if self.equip_menu.left_selected > 0 {
            if self.equip_menu.left_selected == (Item::Cigs as usize) {
                self.effect.update(self.equip_menu.left_selected);

                let now = std::time::Instant::now();
                let elapsed = now - self.last_effect_update;
                if elapsed >= Duration::from_millis(1000) {
                    if self.health > 1.0 {
                        self.health -= 1.0;
                        eprintln!("Health: {:?}", self.health);
                    }
                    self.last_effect_update = now;
                }
            }
        }
    }

    pub fn draw(&mut self) {
        // Set Src
        let src_rect = match self.state {
            State::Standing => {
                match self.direction {
                    Direction::Down => Rect::new(0.0, 0.0, FS_STANDING.x, FS_STANDING.y),
                    Direction::Left => Rect::new(0.0, FS_STANDING.y, FS_STANDING.x, FS_STANDING.y),
                    Direction::Up =>
                        Rect::new(0.0, FS_STANDING.y * 2.0, FS_STANDING.x, FS_STANDING.y),
                    Direction::Right =>
                        Rect::new(0.0, FS_STANDING.y * 3.0, FS_STANDING.x, FS_STANDING.y),
                }
            }
            State::StandingGun => {
                match self.direction {
                    Direction::Down => Rect::new(0.0, 0.0, FS_STANDING_GUN.x, FS_STANDING_GUN.y),
                    Direction::Left =>
                        Rect::new(0.0, FS_STANDING_GUN.y, FS_STANDING_GUN.x, FS_STANDING_GUN.y),
                    Direction::Up =>
                        Rect::new(
                            0.0,
                            FS_STANDING_GUN.y * 2.0,
                            FS_STANDING_GUN.x,
                            FS_STANDING_GUN.y
                        ),
                    Direction::Right =>
                        Rect::new(
                            0.0,
                            FS_STANDING_GUN.y * 3.0,
                            FS_STANDING_GUN.x,
                            FS_STANDING_GUN.y
                        ),
                }
            }
            State::Walking => {
                let frame = (self.frame_counter % MF_WALKING) as f32;
                match self.direction {
                    Direction::Down =>
                        Rect::new(FS_WALKING.x * frame, 0.0, FS_WALKING.x, FS_WALKING.y),
                    Direction::Left =>
                        Rect::new(FS_WALKING.x * frame, FS_WALKING.y, FS_WALKING.x, FS_WALKING.y),
                    Direction::Up =>
                        Rect::new(
                            FS_WALKING.x * frame,
                            FS_WALKING.y * 2.0,
                            FS_WALKING.x,
                            FS_WALKING.y
                        ),
                    Direction::Right =>
                        Rect::new(
                            FS_WALKING.x * frame,
                            FS_WALKING.y * 3.0,
                            FS_WALKING.x,
                            FS_WALKING.y
                        ),
                }
            }
            State::WalkingGun => {
                let frame = (self.frame_counter % MF_WALKING_GUN) as f32;
                match self.direction {
                    Direction::Down =>
                        Rect::new(
                            FS_WALKING_GUN.x * frame,
                            0.0,
                            FS_WALKING_GUN.x,
                            FS_WALKING_GUN.y
                        ),
                    Direction::Left =>
                        Rect::new(
                            FS_WALKING_GUN.x * frame,
                            FS_WALKING_GUN.y,
                            FS_WALKING_GUN.x,
                            FS_WALKING_GUN.y
                        ),
                    Direction::Up =>
                        Rect::new(
                            FS_WALKING_GUN.x * frame,
                            FS_WALKING_GUN.y * 2.0,
                            FS_WALKING_GUN.x,
                            FS_WALKING_GUN.y
                        ),
                    Direction::Right =>
                        Rect::new(
                            FS_WALKING_GUN.x * frame,
                            FS_WALKING_GUN.y * 3.0,
                            FS_WALKING_GUN.x,
                            FS_WALKING_GUN.y
                        ),
                }
            }
        };

        // Set dest
        self.bounds = Rect::new(
            self.position.x,
            self.position.y,
            src_rect.w * SCALE,
            src_rect.h * SCALE
        );

        // Set collider based on destination
        self.collider = Rect::new(
            self.bounds.x,
            self.bounds.y + self.bounds.h * 0.4,
            self.bounds.w,
            self.bounds.h * 0.6
        );

        // Draw
        if self.settings.debug {
            draw_rectangle(
                self.collider.x,
                self.collider.y,
                self.collider.w,
                self.collider.h,
                Color::new(0.0, 1.0, 0.0, 0.5)
            );
        }

        draw_texture_ex(self.texture, self.bounds.x, self.bounds.y, WHITE, DrawTextureParams {
            source: Some(src_rect),
            dest_size: Some(self.bounds.size()),
            ..Default::default()
        });

        if self.equip_menu.left_selected > 0 {
            if self.equip_menu.left_selected == (Item::Cigs as usize) {
                self.effect.draw(self.bounds, self.equip_menu.left_selected);
            }
        }

        // Bullets
        for bullet in &self.bullets {
            bullet.draw();
        }
    }

    // Private
    fn update_frame_counter(&mut self) {
        // Update time vars
        let now = std::time::Instant::now();
        let elapsed = now - self.last_frame_update;

        // Get frame limits
        let max_frames = match self.state {
            State::Standing => 0,
            State::Walking => MF_WALKING,
            State::StandingGun => 0,
            State::WalkingGun => MF_WALKING_GUN,
        };

        // Check frame vs time
        if elapsed >= self.frame_delay {
            self.last_frame_update = now;
            let frames = elapsed.as_secs_f32() / self.frame_delay.as_secs_f32();
            self.frame_counter = (self.frame_counter + (frames as u32)) % max_frames;
        }
    }

    async fn fire_weapon(&mut self) {
        let size = match Weapon::from_index(self.equip_menu.right_selected) {
            Some(Weapon::Empty) => Vec2::new(0.0, 0.0),
            Some(Weapon::Handgun) => Vec2::new(64.0, 64.0),
            None => todo!(),
        };

        let position = match self.direction {
            Direction::Down => Vec2::new(self.bounds.center().x, self.bounds.center().y),
            Direction::Left => Vec2::new(self.collider.x, self.collider.y - 4.0),
            Direction::Up => Vec2::new(self.bounds.center().x, self.position.y),
            Direction::Right => Vec2::new(self.collider.x + self.collider.w, self.collider.y - 4.0),
        };

        let direction = match self.direction {
            Direction::Down => Vec2::new(0.0, 1.0),
            Direction::Left => Vec2::new(-1.0, 0.0),
            Direction::Up => Vec2::new(0.0, -1.0),
            Direction::Right => Vec2::new(1.0, 0.0),
        };

        if self.equip_menu.right_selected == (Weapon::Handgun as usize) {
            eprintln!("Health: {:?}", self.health);
            let bullet = Bullet::new(position, direction, size).await;
            self.bullets.push(bullet);
        }
    }
}