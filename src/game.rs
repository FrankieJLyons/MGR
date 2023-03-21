use macroquad::prelude::*;

pub mod player;
use self::player::Player;

pub struct Game {
    player: Player,
}

impl Game {
    pub async fn new() -> Result<Self, FileError> {
        let player = Player::new().await;
        Ok(Self { player })
    }

    pub fn update(&mut self) {
        self.player.update();
    }

    pub fn draw(&self) {
        self.player.draw();
    }
}