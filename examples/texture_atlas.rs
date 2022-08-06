use bevy::{
    asset::LoadState,
    prelude::*,
    render::texture::ImageSettings,
};

fn main() {
    App::new()
        .init_resource::<SpriteHandles>()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Setup)
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup))
        .run();
}

#[derive(Default)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Finished,
}

fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    sprite_handles.handles = asset_server.load_folder("character/sheet").unwrap();
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(AppState::Finished).unwrap();
    }
}

fn setup(
    mut commands: Commands,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in &sprite_handles.handles {
        let handle = handle.typed_weak();
        let texture = textures.get(&handle).expect("Textures folder contained a file which way matched by a loader which did not create an `Image` asset");
        texture_atlas_builder.add_texture(handle, texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let texture_atlas_texture = texture_atlas.texture.clone();

    let vendor_handle = asset_server.get_handle("character/sheet/down.png");
    let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();

    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(150.0, 0.0, 0.0),
            scale: Vec3::splat(4.0),
            ..default()
        },
        sprite: TextureAtlasSprite::new(vendor_index),
        texture_atlas: atlas_handle,
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: texture_atlas_texture,
        transform: Transform::from_xyz(-300.0, 0.0, 0.0),
        ..default()
    });
}
