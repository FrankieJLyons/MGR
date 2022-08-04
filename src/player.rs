use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

// use crate::image::{self, spawn_image_sprite, ImageSheet};
use crate::image::spawn_image_sprite;
use crate::SNAKE_SIZE;

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            // .add_system(animate_sprite)
            .add_system(move_player);
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
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();
    let size: f32 = SNAKE_SIZE.x / SNAKE_SIZE.y;
    let speed: f32 = player.speed * size * time.delta_seconds();

    if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
        transform.translation.y += speed
    }
    if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
        transform.translation.y -= speed
    }
    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
        transform.translation.x -= speed
    }
    if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
        transform.translation.x += speed
    }
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
