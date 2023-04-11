use macroquad::prelude::*;

pub mod player;
pub mod map;

use crate::game::player::Direction;

use self::map::room::Room;
use self::player::{Player, OFFSET_COL_POS};
use self::map::Map;

pub struct Game {
    player: Player,
    map: Map,
    current_room: Room,
    time_since_last_check: f32,
    check_interval: f32
}

impl Game {
    pub async fn new() -> Result<Self, FileError> {
        let player = Player::new().await;
        let map = Map::new("assets/rooms/arrays/b1_f1.txt").await;

        let rooms = &map.rooms;
        let found_room = rooms.iter().find(|room| room.bounds.contains(player.collider.center()));
        let current_room = found_room.unwrap().clone();
        
        Ok(Self { player, map, current_room, time_since_last_check: 0.0, check_interval: 1.0})
    }

    pub fn update(&mut self) {      
        self.player.update();

        self.room_getter(get_frame_time());
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

    fn room_getter(&mut self, delta_time: f32) {
        self.time_since_last_check += delta_time;

        if self.time_since_last_check >= self.check_interval {
            if !self.current_room.bounds.contains(self.player.collider.center()) {
                let rooms = &self.map.rooms;
                let found_room = rooms.iter().find(|room| room.bounds.contains(self.player.collider.center()));
                self.current_room = found_room.unwrap().clone();
            }
        }

    }

    fn room_collision(&mut self) {
        let colliders = &self.current_room.collider_map.colliders;

        for collider in colliders {
            if collider.overlaps(&self.player.collider) { 
                // Easy maths
                let right = collider.x + collider.w;
                let bottom = collider.y + collider.h;
                let buffer = 0.5;

                let col_up = Vec2::new(collider.center().x, collider.y);
                let col_down = Vec2::new(collider.center().x, bottom);
                let col_left = Vec2::new(collider.x, collider.center().y);
                let col_right = Vec2::new(right, collider.center().y);

                let distances = [
                    self.player.collider.center().distance(col_down),
                    self.player.collider.center().distance(col_up),
                    self.player.collider.center().distance(col_right),
                    self.player.collider.center().distance(col_left),
                ];

                let closest_index = distances
                    .iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                    .map(|(i, _)| i)
                    .unwrap();

                if closest_index == 0 {
                    self.player.position.y = bottom - OFFSET_COL_POS + buffer;
                    self.player.col_arr[0] = true;
                } 

                else if closest_index == 1 {
                    self.player.position.y = collider.y - self.player.bounds.h - buffer;
                    self.player.col_arr[1] = true;
                }
                
                else if closest_index == 2 {
                    self.player.position.x = right + buffer;
                    self.player.col_arr[2] = true;
                }

                else if closest_index == 3 {
                    self.player.position.x = collider.x - self.player.collider.w - buffer;
                    self.player.col_arr[3] = true;
                }

                break;
            } 
        }
    }
}