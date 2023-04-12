use macroquad::prelude::*;

pub mod settings;
pub mod player;
pub mod map;

use self::settings::Settings;
use self::player::Player;
use self::map::Map;
use self::map::room::Room;

pub struct Game {
    settings: Settings,
    player: Player,
    map: Map,
    current_room: Room,
    time_since_last_check: f32,
    check_interval: f32
}

impl Game {
    pub async fn new() -> Result<Self, FileError> {
        let settings = Settings::new();

        let player = Player::new(settings).await;
        let map = Map::new(settings, "assets/rooms/arrays/b1_f1.txt").await;

        let rooms = &map.rooms;
        let found_room = rooms.iter().find(|room| room.bounds.contains(player.collider.center()));
        let current_room = found_room.unwrap().clone();
        
        Ok(Self { settings, player, map, current_room, time_since_last_check: 0.0, check_interval: 1.0})
    }

    pub fn update(&mut self) {   
        self.settings.update();
           
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

        if self.settings.zoom {
            set_camera(&Camera2D {
                zoom: vec2(1.0 / screen_width() / 2.0, -1.0 / screen_height() / 2.0), // half zoom
                target: camera_position,
                ..Default::default()
            });
        } else {
            set_camera(&Camera2D {
                zoom: vec2(1.0 / screen_width() * 2.0, -1.0 / screen_height() * 2.0), // full view
                target: camera_position,
                ..Default::default()
            });
        }
        
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
                let collider_right = collider.x + collider.w;
                let collider_bottom = collider.y + collider.h;
                let buffer = 0.5;

                let distances = [
                    self.player.collider.center().distance(Vec2::new(collider.center().x, collider_bottom)),
                    self.player.collider.center().distance(Vec2::new(collider.center().x, collider.y)),
                    self.player.collider.center().distance(Vec2::new(collider_right, collider.center().y)),
                    self.player.collider.center().distance(Vec2::new(collider.x, collider.center().y)),
                ];

                let closest_index = distances
                    .iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                    .map(|(i, _)| i)
                    .unwrap();

                if closest_index == 0 {
                    self.player.position.y = collider_bottom - self.player.collider.h + buffer;
                    self.player.col_arr[0] = true;
                } 

                else if closest_index == 1 {
                    self.player.position.y = collider.y - self.player.bounds.h - buffer;
                    self.player.col_arr[1] = true;
                }
                
                else if closest_index == 2 {
                    self.player.position.x = collider_right - self.player.collider.w * 0.1 + buffer;
                    self.player.col_arr[2] = true;
                }

                else if closest_index == 3 {
                    self.player.position.x = collider.x - self.player.bounds.w * 0.9 - buffer;
                    self.player.col_arr[3] = true;
                }
            } 
        }
    }
}