use macroquad::prelude::*;
use std::time::Duration;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Player {
    texture: Texture2D,
    textures: [Texture2D; 2],    
    frame_counter: u32,
    frame_delay: Duration,
    last_frame_update: std::time::Instant,
    state: State,
    pub direction: Direction,
    pub position: Vec2,
    pub speed: f32,
    pub collider: Rect,
    dir_map: HashMap<String, bool>
}

// Enums
#[derive(Debug, Clone)]
enum State {
    Standing,
    Walking,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Down,
    Left,
    Up,
    Right,
}

// Frame Size
const FS_STANDING: Vec2 = Vec2::new(17.0, 30.0);
const FS_WALKING: Vec2 = Vec2::new(17.0, 30.0);

// Max Frames
const MF_WALKING: u32 = 2;

// Conts
const START_POS: Vec2 = Vec2::new(512.0 - FS_STANDING.x / 2.0, 384.0 * 8.5);
const SCALE: f32 = 3.0;
const SPEED: f32 = 2.56;
const SHUTTER: u64 = 224;
const OFFSET_COL_POS: f32 = FS_STANDING.y * SCALE / 2.0;

impl Player {
    // Public
    pub async fn new() -> Self {
        // Load Textures
        let standing_texture = load_texture("assets/snake/standing.png").await.unwrap();
        standing_texture.set_filter(FilterMode::Nearest);

        let walking_texture = load_texture("assets/snake/walking.png").await.unwrap();
        walking_texture.set_filter(FilterMode::Nearest);

        let mut my_map: HashMap<String, bool> = HashMap::new();
        // Add key-value pairs to the map
        my_map.insert(String::from("LEFT"), true);
        my_map.insert(String::from("RIGHT"), true);
        my_map.insert(String::from("UP"), true);
        my_map.insert(String::from("DOWN"), true);

        // Set self
        Self {
            texture: standing_texture,
            textures: [standing_texture, walking_texture],
            state: State::Standing,
            direction: Direction::Up,
            position: START_POS,
            speed: SPEED,
            frame_counter: 0,
            frame_delay: Duration::from_millis(SHUTTER),
            last_frame_update: std::time::Instant::now(),
            collider: Rect::new(START_POS.x, START_POS.y + OFFSET_COL_POS, FS_STANDING.x, FS_STANDING.y / 2.0),
            dir_map: my_map
        }
    }

    pub fn update(&mut self) {
        // Pattern:
        //// State,
        //// Texture,
        //// Input,
        //// Other,
        
        match self.state {
            State::Standing => {
                if self.texture != self.textures[0] {
                    self.texture = self.textures[0];
                }

                if is_key_down(KeyCode::Down)
                    || is_key_down(KeyCode::Left)
                    || is_key_down(KeyCode::Up)
                    || is_key_down(KeyCode::Right)
                {
                    self.state = State::Walking;
                    self.frame_counter = 0;
                }
            }

            State::Walking => {
                if self.texture != self.textures[1] {
                    self.texture = self.textures[1];
                }

                if !is_key_down(KeyCode::Down)
                    && !is_key_down(KeyCode::Left)
                    && !is_key_down(KeyCode::Up)
                    && !is_key_down(KeyCode::Right)
                {
                    self.state = State::Standing;
                    self.frame_counter = 0;
                    self.dir_map.insert(String::from("UP"), true);
                    self.dir_map.insert(String::from("DOWN"), true);
                    self.dir_map.insert(String::from("LEFT"), true);
                    self.dir_map.insert(String::from("RIGHT"), true);
                } else {
                    self.update_frame_counter();
                    if is_key_down(KeyCode::Down) {
                        self.direction = Direction::Down;
                        if self.dir_map.get("DOWN") == Some(&true) {
                            self.position.y += self.speed;
                        }
                        self.dir_map.insert(String::from("UP"), true);
                        self.dir_map.insert(String::from("LEFT"), true);
                        self.dir_map.insert(String::from("RIGHT"), true);
                    } else if is_key_down(KeyCode::Left) {
                        self.direction = Direction::Left;
                        if self.dir_map.get("LEFT") == Some(&true) {
                        self.position.x -= self.speed;
                        }
                        self.dir_map.insert(String::from("DOWN"), true);
                        self.dir_map.insert(String::from("UP"), true);
                        self.dir_map.insert(String::from("RIGHT"), true);
                    } else if is_key_down(KeyCode::Up) {
                        self.direction = Direction::Up;
                        if self.dir_map.get("UP") == Some(&true) {
                            self.position.y -= self.speed;
                        }
                        self.dir_map.insert(String::from("DOWN"), true);
                        self.dir_map.insert(String::from("LEFT"), true);
                        self.dir_map.insert(String::from("RIGHT"), true);
                    } else if is_key_down(KeyCode::Right) {
                        self.direction = Direction::Right;
                        if self.dir_map.get("RIGHT") == Some(&true) {
                            self.position.x += self.speed;
                        }
                        self.dir_map.insert(String::from("DOWN"), true);
                        self.dir_map.insert(String::from("LEFT"), true);
                        self.dir_map.insert(String::from("UP"), true);
                    }
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
                    Direction::Up => Rect::new(0.0, FS_STANDING.y * 2.0, FS_STANDING.x, FS_STANDING.y),
                    Direction::Right => Rect::new(0.0, FS_STANDING.y * 3.0, FS_STANDING.x, FS_STANDING.y),
                }
            },
            State::Walking => {
                let frame = ((self.frame_counter % MF_WALKING)) as f32;
                match self.direction {
                    Direction::Down => Rect::new(FS_WALKING.x * frame, 0.0, FS_WALKING.x, FS_WALKING.y),
                    Direction::Left => Rect::new(FS_WALKING.x * frame, FS_WALKING.y, FS_WALKING.x, FS_WALKING.y),
                    Direction::Up => Rect::new(FS_WALKING.x * frame, FS_WALKING.y * 2.0, FS_WALKING.x, FS_WALKING.y),
                    Direction::Right => Rect::new(FS_WALKING.x * frame, FS_WALKING.y * 3.0, FS_WALKING.x, FS_WALKING.y),
                }
            },
        };

        // Set dest
        let dest_rect = Rect::new(
            self.position.x,
            self.position.y,
            src_rect.w * SCALE,
            src_rect.h * SCALE,
        );

        // Set collider based on destination
        self.collider = Rect::new(
            dest_rect.x,
            dest_rect.y + OFFSET_COL_POS,
            dest_rect.w,
            dest_rect.h / 2.0
        );

        draw_rectangle(
            self.collider.x,
            self.collider.y,
            self.collider.w,
            self.collider.h,
            Color::new(0.0, 1.0, 0.0, 0.5),
        );

        // Draw
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

    pub fn set_collision(&mut self, x: f32, y: f32, dir: String) {
        self.position.x = x;
        self.position.y = y;
        self.dir_map.insert(dir, false);
    }

    // Private
    fn update_frame_counter(&mut self) -> u32 {
        // Update time vars
        let now = std::time::Instant::now();
        let elapsed = now - self.last_frame_update;

        // Get frame limits
        let max_frames = match self.state {
            State::Standing => 0,
            State::Walking => 2
        };

        // Check frame vs time
        if elapsed >= self.frame_delay {
            self.last_frame_update = now;
            let frames = elapsed.as_secs_f32() / self.frame_delay.as_secs_f32();
            self.frame_counter = (self.frame_counter + frames as u32) % max_frames;
        }

        // Return current frame
        self.frame_counter
    }

    fn clear_blockers(&mut self, str: String) {
        for (key, _value) in self.dir_map.clone() {
            if key != str {
                self.dir_map.insert(key.to_string(), true);
            }
        }
    }
}