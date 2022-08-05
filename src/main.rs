#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, render::camera::ScalingMode, render::texture::ImageSettings};

mod player;
use player::PlayerPlugin;

mod debug;
use debug::DebugPlugin;

mod image;
use image::ImagePlugin;

mod map;
use map::MapPlugin;

pub const CLEAR: Color = Color::rgb(0.0, 0.0, 0.0);
pub const HEIGHT: f32 = 512.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const ORIGINAL_RESOLUTION: f32 = 4.0 / 3.0;
pub const ORIGINAL_WIDTH: f32 = 256.0;
pub const ORIGINAL_HEIGHT: f32 = 212.0;
pub const SNAKE_SIZE: Vec2 = Vec2::new(16.0, 29.0);
pub const MAP_SIZE: Vec2 = Vec2::new(512.0, 384.0);

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Metal Gear".to_string(),
            resizable: false,
            ..Default::default()
        }) // prevents blurry sprites
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(ImagePlugin)
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    //Set the camera to have normalized coordinates of y values -1 to 1
    camera.projection.top = 100.0;
    camera.projection.bottom = -1.0;

    camera.projection.right = 1.0 * RESOLUTION;
    camera.projection.left = -1.0 * RESOLUTION;

    //Force the camera to use our settings
    camera.projection.scaling_mode = ScalingMode::Auto {
        min_width: (256.0),
        min_height: (212.0),
    };

    commands.spawn_bundle(camera);
}
