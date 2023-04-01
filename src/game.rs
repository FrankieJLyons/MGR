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
        let map = Map::new("assets/rooms/arrays/b1_f1.txt").await;
        
        Ok(Self { player, map })
    }

    pub fn update(&mut self) {
        self.player.update();

        self.camera_update();
    }

    pub fn draw(&mut self) {
        self.map.draw();
        self.player.draw();
    }

    fn camera_update(&self) {
        let camera_position = self.player.position();
        set_camera(&Camera2D {
            zoom: vec2(1.0 / screen_width() / 2.0, -1.0 / screen_height() / 2.0), // half zoom
            //zoom: vec2(1.0 / screen_width() * 2.0, -1.0 / screen_height() * 2.0), // full view
            target: camera_position,
            ..Default::default()
        });
    }
}