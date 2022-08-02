use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    let height = 450.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Metal Gear".to_string(),
            present_mode: PresentMode::AutoNoVsync,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.top = 1.0;
    camera.projection.bottom = -1.0;
    camera.projection.top = 1.0 * RESOLUTION;
    camera.projection.top = -1.0 * RESOLUTION;
    camera.projection.scaling_mode = ScalingMode::None;
    commands.spawn_bundle(camera);
}
