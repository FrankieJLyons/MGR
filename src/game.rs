use macroquad::prelude::*;

pub mod bullet;
pub mod collidermap;
pub mod effect;
pub mod equipmenu;
pub mod map;
pub mod player;
pub mod room;
pub mod settings;

use self::bullet::Bullet;
use self::collidermap::ColliderMap;
use self::effect::Effect;
use self::equipmenu::EquipMenu;
use self::map::Map;
use self::player::Player;
use self::room::Room;
use self::settings::Settings;

pub struct Game {
    settings: Settings,
    player: Player,
    map: Map,
    current_room: Room,
    camera_position: Vec2,
    time_since_last_check: f32,
    check_interval: f32,
    delta_time: f32,
}

impl Game {
    pub async fn new() -> Result<Self, macroquad::Error> {
        let settings = Settings::new();

        let player = Player::new(settings).await;
        let map = Map::new(settings, "assets/rooms/arrays/b1_f1.txt").await;

        let rooms = &map.rooms;
        let found_room = rooms
            .iter()
            .find(|room| room.bounds.contains(player.collider.center()));
        let current_room = found_room.unwrap().clone();

        let camera_position = Vec2 { x: 0.0, y: 0.0 };

        Ok(Self {
            settings,
            player,
            map,
            current_room,
            camera_position,
            time_since_last_check: 0.0,
            check_interval: 1.0,
            delta_time: 0.0,
        })
    }

    pub async fn update(&mut self) {
        self.delta_time = Game::get_delta_time();
        self.settings.update();
        self.player.update_equipment();

        if !self.player.equip_menu.pause {
            self.player.update(self.delta_time).await;

            self.room_getter(get_frame_time());
            self.room_collision();

            self.camera_update();
        }
    }

    pub fn draw(&mut self) {
        self.map.draw();
        self.player.draw();
        self.player.equip_menu.draw(self.camera_position);

        if self.settings.debug {
            draw_text(
                &format!("FPS: {:?}", get_fps()),
                self.player.position.x,
                self.player.position.y,
                64.0,
                WHITE,
            );
            // eprintln!("FPS: {:?}", get_fps());
        }
    }

    fn get_delta_time() -> f32 {
        static mut LAST_FRAME_TIME: Option<std::time::Instant> = None;

        let current_time = std::time::Instant::now();
        let delta_time = match unsafe { LAST_FRAME_TIME } {
            Some(last_frame_time) => current_time.duration_since(last_frame_time).as_secs_f32(),
            None => 0.0,
        };

        unsafe {
            LAST_FRAME_TIME = Some(current_time);
        }

        delta_time
    }

    fn camera_update(&mut self) {
        self.camera_position = self.player.position;

        if self.settings.zoom {
            set_camera(
                &(Camera2D {
                    zoom: vec2(1.0 / screen_width() / 2.0, 1.0 / screen_height() / 2.0), // half zoom
                    target: self.camera_position,
                    ..Default::default()
                }),
            );
        } else {
            set_camera(
                &(Camera2D {
                    zoom: vec2((1.0 / screen_width()) * 2.0, (1.0 / screen_height()) * 2.0), // full view
                    target: self.camera_position,
                    ..Default::default()
                }),
            );
        }
    }

    fn room_getter(&mut self, delta_time: f32) {
        self.time_since_last_check += delta_time;

        if self.time_since_last_check >= self.check_interval {
            if !self
                .current_room
                .bounds
                .contains(self.player.collider.center())
            {
                let rooms = &self.map.rooms;
                let found_room = rooms
                    .iter()
                    .find(|room| room.bounds.contains(self.player.collider.center()));
                if found_room.is_some() {
                    self.current_room = found_room.unwrap().clone();
                } else {
                    eprintln!("OOB: {:?}", self.player.position);
                }
            }
        }
    }

    fn room_collision(&mut self) {
        let colliders = &self.current_room.collider_map.colliders;

        for collider in colliders {
            if collider.overlaps(&self.player.collider) {
                // Easy maths
                let collider_r = collider.x + collider.w;
                let collider_b = collider.y + collider.h;
                let buffer = 0.05;

                let distances = [
                    self.player
                        .collider
                        .center()
                        .distance(Vec2::new(collider.center().x, collider_b)),
                    self.player
                        .collider
                        .center()
                        .distance(Vec2::new(collider.center().x, collider.y)),
                    self.player
                        .collider
                        .center()
                        .distance(Vec2::new(collider_r, collider.center().y)),
                    self.player
                        .collider
                        .center()
                        .distance(Vec2::new(collider.x, collider.center().y)),
                ];

                let closest_index = distances
                    .iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                    .map(|(i, _)| i)
                    .unwrap();

                if closest_index == 0 {
                    self.player.position.y = collider_b - self.player.collider.h * 0.66 + buffer;
                } else if closest_index == 1 {
                    self.player.position.y = collider.y - self.player.bounds.h - buffer;
                } else if closest_index == 2 {
                    self.player.position.x = collider_r + buffer;
                } else if closest_index == 3 {
                    self.player.position.x = collider.x - self.player.bounds.w - buffer;
                }
                self.player.col_arr[closest_index] = true;
                break;
            }
        }
    }
}
