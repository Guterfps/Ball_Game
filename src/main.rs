
use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // this is the player sprite size
pub const NUM_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // this is the player sprite size

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup,
                 (spawn_player, 
                    spawn_camera, 
                    spawn_enemies)
                )
    .add_systems(Update,
                 ((player_movement, 
                    confine_player_movement).chain(),
                    (enemy_movement, 
                    update_enemy_direction,
                    confine_enemy_movement).chain(),
                    enemy_hit_playr)
            )
    .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,

}

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
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            }
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
        
        confine_actor_axis(&mut translation.x, window.width(), PLAYER_SIZE);
        confine_actor_axis(&mut translation.y, window.height(), PLAYER_SIZE);

        player_transform.translation = translation;

    }
}

fn confine_actor_axis(axis: &mut f32, window_size: f32, actor_size: f32) {
    let half_actor_size = actor_size / 2.0;
    let min = half_actor_size;
    let max = window_size - half_actor_size;

    if *axis < min {
        *axis = min;
    } else if *axis > max {
        *axis = max;
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for (mut transform, eneny) in enemy_query.iter_mut() {
        let direction = Vec3::new(eneny.direction.x, eneny.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let min_x = half_enemy_size;
    let max_x = window.width() - half_enemy_size;
    let min_y = half_enemy_size;
    let max_y = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        let mut is_direction_changed = false;
        
        if translation.x <= min_x || translation.x >= max_x {
            enemy.direction.x *= -1.0;
            is_direction_changed = true;
        }
        if translation.y <= min_y || translation.y >= max_y {
            enemy.direction.y *= -1.0;
            is_direction_changed = true;
        }

        if is_direction_changed {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
            
            let sound_effect = if random::<bool>() {
                sound_effect_1
            } else {
                sound_effect_2
            };

            commands.spawn(AudioBundle {
                source: sound_effect,
                settings: PlaybackSettings::DESPAWN,
            });
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    
    for mut enemy_transform in enemy_query.iter_mut() {
        let mut translation = enemy_transform.translation;
        
        confine_actor_axis(&mut translation.x, window.width(), ENEMY_SIZE);
        confine_actor_axis(&mut translation.y, window.height(), ENEMY_SIZE);

        enemy_transform.translation = translation;
    }
    
}

pub fn enemy_hit_playr(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform.translation
                                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance < (player_radius + enemy_radius) {
                println!("Enemy hit player! Game Over");

                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                commands.spawn(AudioBundle {
                    source: sound_effect,
                    settings: PlaybackSettings::DESPAWN,
                });
                commands.entity(player_entity).despawn();
            }
        }
    }
}