use bevy::prelude::*;

pub struct ImagePlugin;

// pub struct ImageSheet(pub Handle<TextureAtlas>);

impl Plugin for ImagePlugin {
    fn build(&self, app: &mut App) {}
}

pub fn spawn_image_sprite(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    path: &str,
    translation: Vec3,
    tile_size: Vec2,
    columns: usize,
    rows: usize,
    padding: Vec2,
    offset: Vec2,
) -> Entity {
    let texture = asset_server.load(path);
    let asset =
        TextureAtlas::from_grid_with_padding(texture, tile_size, columns, rows, padding, offset);
    let atlas_handle = atlases.add(asset);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: atlas_handle,
            transform: Transform {
                translation: translation,
                scale: Vec3::splat(1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

// fn load_image(
//     mut commands: Commands,
//     assets: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
//     path: String,
//     tile_size: Vec2,
//     columns: usize,
//     rows: usize,
//     padding: Vec2,
//     offset: Vec2,
// ) {
//     let texture = assets.load(&path);
//     let asset =
//         TextureAtlas::from_grid_with_padding(texture, tile_size, columns, rows, padding, offset);

//     let atlas_handle = texture_atlases.add(asset);

//     commands.insert_resource(ImageSheet(atlas_handle));
// }
