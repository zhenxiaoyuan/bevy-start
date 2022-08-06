use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*
};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(character_movement_system)
    .add_system(keyboard_input_system)
    .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("character/single/up.png"),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..default()
    }).insert(Direction::Up);
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("character/single/down.png"),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..default()
    }).insert(Direction::Down);
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("character/single/left.png"),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..default()
    }).insert(Direction::Left);
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("character/single/right.png"),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..default()
    }).insert(Direction::Right);
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("character/single/down.png"),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..default()
    }).insert(Direction::None);
}

fn character_movement_system(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (direction, mut transform) in &mut sprite_position {
        match *direction {
            Direction::Up => transform.translation.y += 100.0 * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 100.0 * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 100.0 * time.delta_seconds(),
            Direction::Right => transform.translation.x += 100.0 * time.delta_seconds(),
            Direction::None => ()
        }
    }
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut sprite: Query<&mut Direction>) {
    for mut direction in &mut sprite {
        if keyboard_input.pressed(KeyCode::W) {
            *direction = Direction::Up;
            info!("W pressed");
        } else if keyboard_input.pressed(KeyCode::S) {
            *direction = Direction::Down;
            info!("S pressed");
        } else if keyboard_input.pressed(KeyCode::A) {
            *direction = Direction::Left;
            info!("A pressed");
        } else if keyboard_input.pressed(KeyCode::D) {
            *direction = Direction::Right;
            info!("D pressed");
        } else {
            *direction = Direction::None;
        }
    }
}