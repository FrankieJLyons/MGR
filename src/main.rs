#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, render::camera::ScalingMode, render::texture::ImageSettings};

mod player;
use player::PlayerPlugin;

pub const CLEAR: Color = Color::rgb(0.0, 0.0, 0.0);
pub const HEIGHT: f32 = 512.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const ORIGINAL_RESOLUTION: f32 = 4.0 / 3.0;
pub const ORIGINAL_WIDTH: f32 = 256.0;
pub const ORIGINAL_HEIGHT: f32 = 212.0;
pub const SNAKE_TILE: Vec2 = Vec2::new(16.0, 29.0);

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
        .add_startup_system_to_stage(StartupStage::PreStartup, load_map)
        .add_plugins(DefaultPlugins)
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

struct Map(Handle<TextureAtlas>);

fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = asset_server.load("rooms/MGEAR1_0000.png");
    let atlas = TextureAtlas::new_empty(image, Vec2::new(ORIGINAL_WIDTH, ORIGINAL_HEIGHT));
    let atlas_handle = atlases.add(atlas);

    commands.insert_resource(Map(atlas_handle));
}
