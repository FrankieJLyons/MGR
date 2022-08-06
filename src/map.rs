use image::GenericImageView;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;

pub const RESOLUTION: f32 = 16.0 / 9.0;
// pub const ORIGINAL_RESOLUTION: f32 = 4.0 / 3.0;
pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 450.0;
pub const ORIGINAL_HEIGHT: f32 = 192.0;
pub const ORIGINAL_WIDTH: f32 = 256.0;
pub const MAP_WIDTH: f32 = 512.0;
pub const MAP_HEIGHT: f32 = 384.0;
pub const MAP_SIZE: Vec2 = Vec2::new(MAP_WIDTH, MAP_HEIGHT);
pub const COLLIDE_SIZE: f32 = 8.0;

pub struct MapPlugin;

#[derive(Component)]
pub struct TileCollider;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_map)
            .add_startup_system(create_collision_map);
    }
}

fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture = asset_server.load("rooms/main/000.png");
    let asset = TextureAtlas::from_grid(texture, MAP_SIZE, 1, 1);
    let atlas_handle = atlases.add(asset);

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: atlas_handle,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::splat(0.5),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn create_collision_map(mut commands: Commands) {
    let file = File::open("assets/rooms/collisions/000.txt").expect("No map file found");
    let mut tiles = Vec::new();

    let starting_x = -ORIGINAL_WIDTH / 2.0 + COLLIDE_SIZE / 2.0;
    let starting_y = ORIGINAL_HEIGHT / 2.0 - COLLIDE_SIZE / 2.0;

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile_x = starting_x + x as f32 * COLLIDE_SIZE;
                let tile_y = starting_y - y as f32 * COLLIDE_SIZE;
                let z = 100.0;

                let translation = Vec3::new(tile_x, tile_y, z);
                let mut color = Color::rgba(1.0, 1.0, 1.0, 0.0);
                let tile = spawn_tile(&mut commands, translation);

                if char == 'x' {
                    color = Color::rgba(1.0, 1.0, 1.0, 0.5);
                    commands.entity(tile).insert(TileCollider);
                }

                tiles.push(tile);

                // collision tile
                commands.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: color,
                        ..default()
                    },
                    transform: Transform {
                        translation: translation,
                        scale: Vec3::splat(COLLIDE_SIZE),
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }

    commands
        .spawn()
        .insert(Name::new("CollisionMap"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}

pub fn spawn_tile(commands: &mut Commands, translation: Vec3) -> Entity {
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}
