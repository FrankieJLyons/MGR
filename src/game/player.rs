use macroquad::prelude::*;
use std::time::Duration;

pub struct Player {
    texture: Texture2D,
    textures: [Texture2D; 2],    
    state: State,
    direction: Direction,
    position: Vec2,
    speed: f32,
    frame_counter: u32,
    frame_delay: Duration,
    last_frame_update: std::time::Instant,
}

enum State {
    Standing,
    Walking,
}

enum Direction {
    Down,
    Left,
    Up,
    Right,
}

const SCALE: f32 = 3.0;

const FS_STANDING: Vec2 = Vec2::new(17.0, 30.0);
const FS_WALKING: Vec2 = Vec2::new(17.0, 30.0);

const MF_WALKING: u32 = 2;

impl Player {
    // Public
    pub async fn new() -> Self {
        let standing_texture = load_texture("assets/snake/standing.png").await.unwrap();
        standing_texture.set_filter(FilterMode::Nearest);

        let walking_texture = load_texture("assets/snake/walking.png").await.unwrap();
        walking_texture.set_filter(FilterMode::Nearest);

        Self {
            texture: standing_texture,
            textures: [standing_texture, walking_texture],
            state: State::Standing,
            direction: Direction::Down,
            position: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
            speed: 2.56,
            frame_counter: 0,
            frame_delay: Duration::from_millis(224),
            last_frame_update: std::time::Instant::now()
        }
    }

    pub fn update(&mut self) {
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
                } else {
                    self.update_frame_counter();
                    if is_key_down(KeyCode::Down) {
                        self.direction = Direction::Down;
                        self.position.y += self.speed;
                    } else if is_key_down(KeyCode::Left) {
                        self.direction = Direction::Left;
                        self.position.x -= self.speed;
                    } else if is_key_down(KeyCode::Up) {
                        self.direction = Direction::Up;
                        self.position.y -= self.speed;
                    } else if is_key_down(KeyCode::Right) {
                        self.direction = Direction::Right;
                        self.position.x += self.speed;
                    }
                }
            }
        }
    }
    
    pub fn draw(&self) {
        let (src_x, src_y, src_w, src_h) = match self.state {
            State::Standing => {
                match self.direction {
                    Direction::Down => (0.0, 0.0, FS_STANDING.x, FS_STANDING.y),
                    Direction::Left => (0.0, FS_STANDING.y, FS_STANDING.x, FS_STANDING.y),
                    Direction::Up => (0.0, FS_STANDING.y * 2.0, FS_STANDING.x, FS_STANDING.y),
                    Direction::Right => (0.0, FS_STANDING.y * 3.0, FS_STANDING.x, FS_STANDING.y),
                }
            },
            State::Walking => {
                let frame = ((self.frame_counter % MF_WALKING)) as f32;
                match self.direction {
                    Direction::Down => (FS_WALKING.x * frame, 0.0, FS_WALKING.x, FS_WALKING.y),
                    Direction::Left => (FS_WALKING.x * frame, FS_WALKING.y, FS_WALKING.x, FS_WALKING.y),
                    Direction::Up => (FS_WALKING.x * frame, FS_WALKING.y * 2.0, FS_WALKING.x, FS_WALKING.y),
                    Direction::Right => (FS_WALKING.x * frame, FS_WALKING.y * 3.0, FS_WALKING.x, FS_WALKING.y),
                }
            },
        };

        let src_rect = Rect::new(src_x, src_y, src_w, src_h);
        let dest_rect = Rect::new(
            self.position.x,
            self.position.y,
            src_w * SCALE,
            src_h * SCALE,
        );

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

    // Private
    fn update_frame_counter(&mut self) -> u32 {
        let now = std::time::Instant::now();
        let elapsed = now - self.last_frame_update;

        let max_frames = match self.state {
            State::Standing => 0,
            State::Walking => 2
        };

        if elapsed >= self.frame_delay {
            let frames = elapsed.as_secs_f32() / self.frame_delay.as_secs_f32();
            self.frame_counter = (self.frame_counter + frames as u32) % max_frames;
            self.last_frame_update = now;
        }

        self.frame_counter
    }
}