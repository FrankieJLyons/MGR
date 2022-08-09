use std::time::Duration;

use image::GenericImageView;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub const RESOLUTION: f32 = 16.0 / 9.0;
// pub const ORIGINAL_RESOLUTION: f32 = 4.0 / 3.0;
pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 450.0;
pub const ORIGINAL_HEIGHT: f32 = 192.0;
pub const ORIGINAL_WIDTH: f32 = 256.0;
pub const MAP_WIDTH: f32 = 512.0;
pub const MAP_HEIGHT: f32 = 384.0;
pub const MAP_SIZE: Vec2 = Vec2::new(MAP_WIDTH, MAP_HEIGHT);
pub const COLLIDER_SIZE: f32 = 8.0;
pub const MAP_COLLIDER_SIZE: Vec2 = Vec2::new(ORIGINAL_WIDTH, ORIGINAL_HEIGHT);

pub struct MapPlugin;

#[derive(Component)]
pub struct MapHolder {
    pub timer: Timer,
}

#[derive(Component, Inspectable)]
pub struct Map {
    pub name: String,
}

#[derive(Component)]
struct ColliderMap;

#[derive(Component)]
pub struct Collider;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_map)
            .add_startup_system(create_collision_map);
    }
}

fn create_map(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut maps = Vec::new();

    for i in 0..2 {
        let path = format!("rooms/main/00{}.png", i);
        let translation = Vec3::new(0.0, ORIGINAL_HEIGHT * i as f32, 0.0);
        let name = format!("00{}", i);
        let map_file = spawn_map(
            &mut commands,
            &mut asset_server,
            &mut atlases,
            &path,
            translation,
            name,
        );
        maps.push(map_file);
    }

    commands
        .spawn()
        .insert(MapHolder {
            timer: Timer::new(Duration::from_secs(3), true),
        })
        .insert(Name::new("MapHolder"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Visibility::default())
        .insert(ComputedVisibility::default())
        .push_children(&maps);

    // Get the map image player is currently over - done
    // Get that images name - done
    // Do a look up of a dataset for that maps name, and get the names of adjacent maps and their positions
    // Load said maps
}

fn spawn_map(
    commands: &mut Commands,
    asset_server: &mut Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
    path: &str,
    translation: Vec3,
    name: String,
) -> Entity {
    let texture = asset_server.load(&*path);
    let asset = TextureAtlas::from_grid(texture, MAP_SIZE, 1, 1);
    let atlas_handle = atlases.add(asset);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: atlas_handle,
            transform: Transform {
                translation: translation,
                scale: Vec3::splat(0.5),
                ..default()
            },
            ..default()
        })
        .insert(Map { name: name })
        .id()
}

fn create_collision_map(mut commands: Commands) {
    let img = image::open("assets/rooms/collisions/000.png").unwrap();
    let mut tiles = Vec::new();
    let starting_x = -ORIGINAL_WIDTH / 2.0 + COLLIDER_SIZE / 2.0;
    let starting_y = ORIGINAL_HEIGHT / 2.0 - COLLIDER_SIZE / 2.0;

    for pixel in img.pixels() {
        let x = starting_x + pixel.0 as f32 * COLLIDER_SIZE;
        let y = starting_y - pixel.1 as f32 * COLLIDER_SIZE;
        let z = 100.0;
        let color = Color::rgb(pixel.2[0] as f32, pixel.2[0] as f32, pixel.2[0] as f32);

        let translation = Vec3::new(x, y, z);
        let tile = spawn_tile(&mut commands, translation, color);

        if color == Color::BLACK {
            commands.entity(tile).insert(Collider);
        }
        tiles.push(tile);

        // println!("{:?}", pixel);
    }

    commands
        .spawn()
        .insert(ColliderMap)
        .insert(Name::new("ColliderMap"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Visibility::default())
        .insert(ComputedVisibility::default())
        .push_children(&tiles);
}

pub fn spawn_tile(commands: &mut Commands, translation: Vec3, color: Color) -> Entity {
    let mut display_color = Color::rgba(1.0, 1.0, 1.0, 0.0);
    if color == Color::BLACK {
        display_color = Color::rgba(1.0, 1.0, 1.0, 0.25);
    }

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: display_color,
                ..default()
            },
            transform: Transform {
                translation: translation,
                scale: Vec3::splat(COLLIDER_SIZE),
                ..default()
            },
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .id()
}
