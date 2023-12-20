
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // this is the player sprite size
pub const NUM_OF_ENEMIES: usize = 4;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup,
                 (spawn_player, spawn_camera, spawn_enemies))
    .add_systems(Update,
                 (player_movement, confine_player_movement))
    .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, 
                                        window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {}
    ));
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, 
                                        window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUM_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {}
        ));
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || 
            keyboard_input.pressed(KeyCode::A) {
                direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) || 
            keyboard_input.pressed(KeyCode::D) {
                direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) || 
            keyboard_input.pressed(KeyCode::W) {
                direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) || 
            keyboard_input.pressed(KeyCode::S) {
                direction.y -= 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();

    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let mut translation = player_transform.translation;
        
        confine_player_axis(&mut translation.x, window.width());
        confine_player_axis(&mut translation.y, window.height());

        player_transform.translation = translation;

    }
}

fn confine_player_axis(axis: &mut f32, window_size: f32) {
    let half_player_size = PLAYER_SIZE / 2.0;
    let min = half_player_size;
    let max = window_size - half_player_size;

    if *axis < min {
        *axis = min;
    } else if *axis > max {
        *axis = max;
    }
}