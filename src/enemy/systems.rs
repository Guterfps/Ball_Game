
use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::enemy::{
    components::*, 
    resources::*,
    NUM_OF_ENEMIES,
    ENEMY_SIZE,
    ENEMY_SPEED,
    confine_actor_axis  
};

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    (0..NUM_OF_ENEMIES).for_each(|_| {
        enemy_spawn(window, &mut commands, &asset_server);
    });
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

pub fn tick_enemy_spawn_timer(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>
) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        enemy_spawn(window, &mut commands, &asset_server);
    }
}


fn enemy_spawn(
    window: &Window,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) {
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