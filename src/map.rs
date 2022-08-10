use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, ptr::null, time::Duration};

use bevy::{prelude::*, reflect::Array};
use bevy_inspector_egui::{egui::epaint::tessellator::Path, Inspectable};

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
    pub current: String,
    pub siblings: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Component, Inspectable)]
pub struct Map {
    pub name: String,
    pub siblings: Vec<String>,
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
struct ColliderHolder;

#[derive(Component)]
pub struct Collider;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_opening_map);
    }
}

fn create_opening_map(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut maps = Vec::new();
    let name = "121";
    let translation = Vec3::new(ORIGINAL_HEIGHT * 0.0, ORIGINAL_HEIGHT * -1.0, 0.0);
    let path = format!("rooms/side/{}.png", name);
    let map_file = spawn_map(
        &mut commands,
        &mut asset_server,
        &mut atlases,
        name.to_string(),
        translation,
        &path,
    );
    maps.push(map_file);
    create_opening_collision_map(&mut commands, name, translation);

    let mut siblings = Vec::new();
    siblings.push("000".to_string());

    commands
        .spawn()
        .insert(MapHolder {
            timer: Timer::new(Duration::from_secs(3), true),
            current: name.to_string(),
            siblings: siblings,
        })
        .insert(Name::new("MapHolder"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Visibility::default())
        .insert(ComputedVisibility::default())
        .push_children(&maps);
}

fn create_opening_collision_map(commands: &mut Commands, name: &str, origin: Vec3) {
    let img = image::open(format!("assets/rooms/collisions/{}.png", name)).unwrap();
    let mut tiles = Vec::new();
    let starting_x = origin.x - (ORIGINAL_WIDTH / 2.0 - COLLIDER_SIZE / 2.0);
    let starting_y = origin.y + (ORIGINAL_HEIGHT / 2.0 - COLLIDER_SIZE / 2.0);

    for pixel in img.pixels() {
        let x = starting_x + pixel.0 as f32 * COLLIDER_SIZE;
        let y = starting_y - pixel.1 as f32 * COLLIDER_SIZE;
        let z = 100.0;
        let color = Color::rgb(pixel.2[0] as f32, pixel.2[0] as f32, pixel.2[0] as f32);

        let translation = Vec3::new(x, y, z);
        let tile = spawn_tile(commands, translation, color);

        if color == Color::BLACK {
            commands.entity(tile).insert(Collider);
        }
        tiles.push(tile);
    }

    commands
        .spawn()
        .insert(ColliderHolder)
        .insert(Name::new("ColliderHolder"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Visibility::default())
        .insert(ComputedVisibility::default())
        .push_children(&tiles);
}

fn create_map(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    mut map_holder_query: Query<&mut MapHolder>,
) {
    // Call initially with starting map
    // Pass in map name
    // Get the maps siblings from some data sheet
    //// prevent siblings from loading their siglings
    // Load more maps based on data
    // Get the map image player is currently over - done
    // Get that images name - done
    // Pass in the new name
    // Do a look up of a dataset and load new maps while despawning old maps

    let mut maps = Vec::new();

    let data = fs::read_to_string("assets/rooms/rooms.json").expect("Unable to read JSON file");
    let json: Vec<Map> = serde_json::from_str(&data).expect("JSON was not well-formatted");

    for room in &json {
        println!("{:?}\n", room);

        let name = format!("{}", room.name);
        let translation = Vec3::new(ORIGINAL_HEIGHT * room.x, ORIGINAL_HEIGHT * room.y, 0.0);
        let path = format!("rooms/main/{}.png", room.name);
        let map_file = spawn_map(
            &mut commands,
            &mut asset_server,
            &mut atlases,
            name,
            translation,
            &path,
        );
        maps.push(map_file);
    }

    let mut siblings = Vec::new();
    siblings.push("000".to_string());

    commands
        .spawn()
        .insert(MapHolder {
            timer: Timer::new(Duration::from_secs(3), true),
            current: "".to_string(),
            siblings: siblings,
        })
        .insert(Name::new("MapHolder"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Visibility::default())
        .insert(ComputedVisibility::default())
        .push_children(&maps);
}

fn spawn_map(
    commands: &mut Commands,
    asset_server: &mut Res<AssetServer>,
    atlases: &mut ResMut<Assets<TextureAtlas>>,
    name: String,
    translation: Vec3,
    path: &str,
) -> Entity {
    let texture = asset_server.load(&*path);
    let asset = TextureAtlas::from_grid(texture, MAP_SIZE, 1, 1);
    let atlas_handle = atlases.add(asset);

    let mut siblings = Vec::new();
    siblings.push("".to_string());

    let position = Vec2::new(0.0, 0.0);

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
        .insert(Map {
            name: name,
            siblings: siblings,
            x: 0.0,
            y: 0.0,
        })
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
        .insert(ColliderHolder)
        .insert(Name::new("ColliderHolder"))
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
