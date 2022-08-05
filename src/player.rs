use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_inspector_egui::Inspectable;

// use crate::image::{self, spawn_image_sprite, ImageSheet};
use crate::image::spawn_image_sprite;
use crate::map::TileCollider;
use crate::{COLLIDE_SIZE, SNAKE_SIZE};

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            // .add_system(animate_sprite)
            .add_system(camera_follow.after("movement"))
            .add_system(move_player.label("movement"));
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player = spawn_image_sprite(
        &mut commands,
        asset_server,
        atlases,
        "snake/snake_walking.png",
        Vec3::new(0.0, 0.0, 900.0),
        SNAKE_SIZE,
        3,
        1,
        Vec2::splat(1.0),
        Vec2::splat(1.0),
    );

    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player { speed: 100.0 });
}

fn move_player(
    mut player_query: Query<(&Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();
    let size: f32 = SNAKE_SIZE.x / SNAKE_SIZE.y;
    let speed: f32 = player.speed * size * time.delta_seconds();

    let mut y_delta = 0.0;
    if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
        y_delta += speed;
    }
    if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
        y_delta -= speed;
    }

    let mut x_delta = 0.0;
    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
        x_delta -= speed;
    }
    if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
        x_delta += speed;
    }

    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
    }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
    }

    // print_data(transform.translation.to_string())
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            Vec3::new(
                target_player_pos.x,
                target_player_pos.y - SNAKE_SIZE.y / 4.0,
                target_player_pos.z,
            ),
            Vec2::new(14.0, 14.0),
            wall_transform.translation,
            Vec2::splat(COLLIDE_SIZE),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

// #[derive(Component, Deref, DerefMut)]
// struct AnimationTimer(Timer);

// fn animate_sprite(
//     time: Res<Time>,
//     texture_atlases: Res<Assets<TextureAtlas>>,
//     mut query: Query<(
//         &mut AnimationTimer,
//         &mut TextureAtlasSprite,
//         &Handle<TextureAtlas>,
//     )>,
// ) {
//     for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
//         timer.tick(time.delta());
//         if timer.just_finished() {
//             let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
//             sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
//         }
//     }
// }
