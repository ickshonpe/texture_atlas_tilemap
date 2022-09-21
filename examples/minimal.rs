use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::window::PresentMode;
use bevy::window::WindowMode;
use texture_atlas_tilemap::prelude::*;

pub fn setup(
    mut commands: Commands
) {
    commands.spawn_bundle(Camera2dBundle::default());
}

pub fn spawn_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture = asset_server.load("textures/tileset.png");
    let atlas = TextureAtlas::from_grid(texture, 16.0 * Vec2::ONE, 2, 2);
    let atlas_handle = texture_atlases.add(atlas);
    let width = 1920 / 8 + 1;
    let height = 1080 / 8;
    commands.spawn_bundle(texture_atlas_tilemap::TextureAtlasTilemapBundle {
        tilemap: TextureAtlasTilemap {
            width,
            height,
            atlas_indices: (0..width * height).map(|i| i % 2).collect(),
        },
        tilemap_geometry: TextureAtlasTilemapGeometry { 
            tile_size: vec2(8.0, 8.0), 
            anchor: bevy::sprite::Anchor::Center
        },
        texture_atlas: atlas_handle,
        ..Default::default()
    });
}

pub fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        present_mode: PresentMode::Immediate,
        mode: WindowMode::Fullscreen,
        ..Default::default()
    })
    .insert_resource(ImageSettings::default_nearest())
    .insert_resource(Msaa { samples: 1 })
    .add_plugins(DefaultPlugins)
    .add_plugin(TextureAtlasTilemapPlugin)
    .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
    .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
    .add_startup_system(setup)
    .add_startup_system(spawn_tilemap)
    .run();
}