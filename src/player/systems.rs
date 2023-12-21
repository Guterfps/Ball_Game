
use bevy::{prelude::*, window::PrimaryWindow};

use crate::player::{
    components::*,
    PLAYER_SIZE,
    PLAYER_SPEED,
};
use crate::enemy::{
    components::Enemy, 
    ENEMY_SIZE,
    confine_actor_axis
};
use crate::score::resources::Score;
use crate::star::{components::Star, STAR_SIZE};
use crate::events::GameOver;

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
        Player {},
    ));
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

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut game_over_event_writer: EventWriter<GameOver>,
    score: Res<Score>
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
                game_over_event_writer.send(GameOver { score: score.value });
                break;
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>
) {
    if let Ok(player_transform) = player_query.get_single() {
        star_query
            .for_each_mut(|(star_entety, star_transform)| {
            let distance = player_transform.translation
                                .distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;

            if distance < (player_radius + star_radius) {
                println!("Player hit star! Star collected");

                let sound_effect = asset_server
                                .load("audio/laserLarge_000.ogg");
                commands.spawn(AudioBundle {
                    source: sound_effect,
                    settings: PlaybackSettings::DESPAWN,
                });
                commands.entity(star_entety).despawn();
                score.value += 1;
            }
        });
    }
}
