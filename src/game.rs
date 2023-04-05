use macroquad::prelude::*;

pub mod player;
pub mod map;

use crate::game::player::Direction;

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

    fn room_collision(&mut self) {
        // find the room the player is currently in
        let rooms = &self.map.rooms;
        let current_room = rooms.iter().find(|room| room.bounds.contains(self.player.collider.center()));

        // check for collisions with the player's collider
        if let Some(room) = current_room {
            let colliders = &room.collider_map.colliders;

            for collider in colliders {
                if collider.overlaps(&self.player.collider) { 

                    //println!("C: {}, {}, {}, {}", collider.x, collider.y, collider.w, collider.h);
                    //println!("P: {}, {}, {}, {}", self.player.collider.x, self.player.collider.y, self.player.collider.w, self.player.collider.h);

                    let mut diff_x = 0.0;
                    let mut diff_y = 0.0;

                    let col_right = collider.x + collider.w;
                    let col_bottom = collider.y + collider.h;
                    let play_right = self.player.collider.x + self.player.collider.w;
                    let play_bottom = self.player.collider.y + self.player.collider.h;

                    let mut dir = String::from("NONE");

                    if self.player.direction == Direction::Left && self.player.collider.x <= col_right  {
                        diff_x = col_right - self.player.collider.x;
                        dir = String::from("LEFT");
                    }

                    else if self.player.direction == Direction::Right && play_right >= collider.x {
                        diff_x = collider.x - play_right;
                        dir = String::from("RIGHT");
                    }

                    else if self.player.direction == Direction::Up && self.player.collider.y <= col_bottom  {
                        diff_y = col_bottom - self.player.collider.y;
                        dir = String::from("UP");
                    }

                    else if self.player.direction == Direction::Down && play_bottom >= collider.y  {
                        diff_y = collider.y - play_bottom;
                        dir = String::from("DOWN");
                    }

                    self.player.set_collision(self.player.position.x + diff_x * 1.01, self.player.position.y + diff_y * 1.01, dir);
                }
                
            }
        } else {
            eprintln!("Out of Bounds: Player Position = {:?}", self.player.position);
        }
    }
}