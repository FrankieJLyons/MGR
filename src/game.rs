use macroquad::prelude::*;

pub mod player;
pub mod map;

use self::player::Player;
use self::map::Map;

pub struct Game {
    player: Player,
    map: Map,
}

impl Game {
    pub async fn new() -> Result<Self, FileError> {
        let player = Player::new().await;
        let map = Map::new().await;
        
        Ok(Self { player, map })
    }

    pub fn update(&mut self) {
        self.player.update();
    }

    pub fn draw(&mut self) {
        self.map.draw();
        self.player.draw();
    }
}