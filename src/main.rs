#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, render::camera::ScalingMode, render::texture::ImageSettings};

use std::io::{stdout, Write};

mod asset;
mod debug;
mod map;
mod player;

use asset::AssetPlugin;
use debug::DebugPlugin;
use map::{MapPlugin, HEIGHT, RESOLUTION, WIDTH};
use player::PlayerPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Play,
    Pause,
    Codec,
    Select,
    Script,
}

fn main() {
    App::new()
        .add_state(GameState::Play)
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Metal Gear".to_string(),
            resizable: false,
            ..Default::default()
        }) // prevents blurry sprites
        .add_startup_system(spawn_camera)
        .add_system(bevy::window::close_on_esc)
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(AssetPlugin)
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

    commands.spawn_bundle(camera).insert(Name::new("Camera"));
}

pub fn print_data(string: String) {
    let mut stdout = stdout();
    print!("\r{}", string);
    stdout.flush().unwrap();
    println!();
}
