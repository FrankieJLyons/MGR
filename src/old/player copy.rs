// use bevy::prelude::*;
// use bevy::sprite::collide_aabb::collide;
// use bevy_inspector_egui::Inspectable;

// use crate::asset::spawn_image_sprite;
// use crate::map::{create_map, Collider, Map, MapHolder, COLLIDER_SIZE, MAP_COLLIDER_SIZE};
// use crate::{print_data, GameState};

// pub const SNAKE_SIZE: Vec2 = Vec2::new(16.0, 29.0);
// pub const SNAKE_OFFSET: f32 = SNAKE_SIZE.y / 4.0;
// pub const SNAKE_COLLIDE_SIZE: Vec2 = Vec2::new(SNAKE_SIZE.x * 0.875, SNAKE_SIZE.y / 2.0);
// pub struct PlayerPlugin;

// #[derive(Component, Inspectable)]
// pub struct Player {
//     speed: f32,
// }

// impl Plugin for PlayerPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_system_set(SystemSet::on_update(GameState::Play))
//             .add_startup_system(spawn_player)
//             // .add_system(animate_sprite)
//             .add_system(player_map_checking.after("movement"))
//             .add_system(camera_follow.after("movement"))
//             .add_system(move_player.label("movement"));
//     }
// }

// fn spawn_player(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     atlases: ResMut<Assets<TextureAtlas>>,
// ) {
//     let player = spawn_image_sprite(
//         &mut commands,
//         asset_server,
//         atlases,
//         "snake/snake_walking.png",
//         Vec3::new(0.0, -96.0, 900.0),
//         SNAKE_SIZE,
//         3,
//         1,
//         Vec2::splat(1.0),
//         Vec2::splat(1.0),
//     );

//     commands
//         .entity(player)
//         .insert(Name::new("Player"))
//         .insert(Player { speed: 100.0 });
// }

// fn move_player(
//     mut player_query: Query<(&Player, &mut Transform)>,
//     wall_query: Query<&Transform, (With<Collider>, Without<Player>)>,
//     keyboard: Res<Input<KeyCode>>,
//     time: Res<Time>,
// ) {
//     let (player, mut transform) = player_query.single_mut();
//     let size: f32 = SNAKE_SIZE.x / SNAKE_SIZE.y;
//     let speed: f32 = player.speed * size * time.delta_seconds();

//     let mut y_delta = 0.0;
//     if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
//         y_delta += speed;
//     }
//     if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
//         y_delta -= speed;
//     }

//     let mut x_delta = 0.0;
//     if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
//         x_delta -= speed;
//     }
//     if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
//         x_delta += speed;
//     }

//     let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
//     if wall_collision_check(target, &wall_query) {
//         transform.translation = target;
//     }

//     let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
//     if wall_collision_check(target, &wall_query) {
//         transform.translation = target;
//     }
// }

// fn wall_collision_check(
//     target_player_pos: Vec3,
//     wall_query: &Query<&Transform, (With<Collider>, Without<Player>)>,
// ) -> bool {
//     for wall_transform in wall_query.iter() {
//         let collision = collide(
//             Vec3::new(
//                 target_player_pos.x,
//                 target_player_pos.y - SNAKE_OFFSET,
//                 target_player_pos.z,
//             ),
//             SNAKE_COLLIDE_SIZE,
//             wall_transform.translation,
//             Vec2::splat(COLLIDER_SIZE),
//         );
//         if collision.is_some() {
//             return false;
//         }
//     }
//     true
// }

// fn player_map_checking(
//     mut commands: Commands,
//     mut asset_server: Res<AssetServer>,
//     mut atlases: ResMut<Assets<TextureAtlas>>,
//     mut map_holder_query: Query<&mut MapHolder>,
//     map_query: Query<(&mut Map, &Transform)>,
//     mut player_query: Query<(&mut Player, &Transform)>,
//     time: Res<Time>,
// ) {
//     let mut map_holder = map_holder_query.single_mut();
//     map_holder.timer.tick(time.delta());
//     if map_holder.timer.finished() {
//         let (_player, player_transform) = player_query.single_mut();
//         let player_translation = player_transform.translation;

//         for map_i in map_query.iter() {
//             let (map, map_transform) = map_i;
//             let map_translation = map_transform.translation;

//             if map_holder.current != map.name.to_string()
//                 && map_collision_check(player_translation, map_translation)
//             {
//                 // despawn maps based on current
//                 // ...
//                 map_holder.current = map.name.to_string();
//                 print_data(map_holder.current.to_string());
//                 // spawn maps based on new current
//                 create_map(
//                     &mut commands,
//                     &mut asset_server,
//                     &mut atlases,
//                     &mut map_holder,
//                 );
//             }
//         }
//     }
// }

// fn map_collision_check(player_translation: Vec3, map_translation: Vec3) -> bool {
//     let collision = collide(
//         player_translation,
//         SNAKE_SIZE,
//         map_translation,
//         MAP_COLLIDER_SIZE,
//     );
//     collision.is_some()
// }

// fn camera_follow(
//     player_query: Query<&Transform, With<Player>>,
//     mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
// ) {
//     let player_transform = player_query.single();
//     let mut camera_transform = camera_query.single_mut();

//     camera_transform.translation.x = player_transform.translation.x;
//     camera_transform.translation.y = player_transform.translation.y;
// }

// // #[derive(Component, Deref, DerefMut)]
// // struct AnimationTimer(Timer);

// // fn animate_sprite(
// //     time: Res<Time>,
// //     texture_atlases: Res<Assets<TextureAtlas>>,
// //     mut query: Query<(
// //         &mut AnimationTimer,
// //         &mut TextureAtlasSprite,
// //         &Handle<TextureAtlas>,
// //     )>,
// // ) {
// //     for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
// //         timer.tick(time.delta());
// //         if timer.just_finished() {
// //             let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
// //             sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
// //         }
// //     }
// // }
