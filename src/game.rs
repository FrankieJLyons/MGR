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

        self.room_collision();

        self.camera_update();
    }

    pub fn draw(&mut self) {
        self.map.draw();
        self.player.draw();
    }

    fn camera_update(&self) {
        let camera_position = self.player.position;
        set_camera(&Camera2D {
            //zoom: vec2(1.0 / screen_width() / 2.0, -1.0 / screen_height() / 2.0), // half zoom
            zoom: vec2(1.0 / screen_width() * 2.0, -1.0 / screen_height() * 2.0), // full view
            target: camera_position,
            ..Default::default()
        });
    }

    fn room_collision(&self) {
        // find the room the player is currently in
        let rooms = &self.map.rooms;
        let current_room = rooms.iter().find(|room| room.bounds.contains(self.player.position));

        // check for collisions with the player's collider
        if let Some(room) = current_room {

            let colliders = &room.collider_map.colliders;
            let player_collider = &self.player.collider;

            for collider in colliders {
                if collider.overlaps(player_collider) {
                    // handle collision
                    eprintln!("Collision: {}", room.name);
                } 
            }
        } else {
            eprintln!("Out of Bounds: Player Position = {:?}", self.player.position);
        }
    }
}