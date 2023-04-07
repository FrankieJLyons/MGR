use macroquad::prelude::*;

pub mod player;
pub mod map;

use crate::game::player::Direction;

use self::player::{Player, OFFSET_COL_POS};
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
                    // Easy maths
                    let col_right = collider.x + collider.w;
                    let col_bottom = collider.y + collider.h;
                    let play_right = self.player.collider.x + self.player.collider.w;
                    let play_bottom = self.player.collider.y + self.player.collider.h;
                    let buffer = 0.5;

                    if self.player.direction == Direction::Up {
                        if self.player.collider.y < col_bottom || self.player.collider.y + self.player.speed < col_bottom {
                            self.player.position.y = col_bottom - OFFSET_COL_POS + buffer;
                            self.player.col_arr[0] = true;
                        } else {
                            self.player.col_arr[0] = false;
                        }
                    } 

                    else if self.player.direction == Direction::Down {
                        if play_bottom > collider.y || play_bottom - self.player.speed > collider.y  {
                            self.player.position.y = collider.y - self.player.bounds.h - buffer;
                            self.player.col_arr[1] = true;
                        } else {
                            self.player.col_arr[1] = false;
                        }
                    }
                    
                    else if self.player.direction == Direction::Left {
                        if self.player.collider.x < col_right || self.player.collider.x + self.player.speed < col_right  {
                            self.player.position.x = col_right + buffer;
                            self.player.col_arr[2] = true;
                        } else {
                            self.player.col_arr[2] = false;
                        }
                    }

                    else if self.player.direction == Direction::Right {
                        if play_right > collider.x || play_right - self.player.speed > collider.x  {
                            self.player.position.x = collider.x - self.player.collider.w - buffer;
                            self.player.col_arr[3] = true;
                        } else {
                            self.player.col_arr[3] = false;
                        }
                    }
                } 
            }
        } else {
            eprintln!("Out of Bounds: Player Position = {:?}", self.player.position);
        }
    }
}