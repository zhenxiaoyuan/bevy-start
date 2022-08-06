use bevy::{prelude::*, render::texture::ImageSettings};

fn main() {
    App::new()
    .insert_resource(ImageSettings::default_nearest())
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(animate_sprite)
    .run();
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("character/down.png");
    let texture_altas = TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 4, 1);
    let texture_altas_handle = texture_atlases.add(texture_altas);

    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(
        SpriteSheetBundle {
            texture_atlas: texture_altas_handle,
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        }
    ).insert(AnimationTimer(Timer::from_seconds(0.25, true)));
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>
    )>
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        if timer.0.tick(time.delta()).just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}