
use bevy::{prelude::*, window::PrimaryWindow, app::AppExit};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // this is the player sprite size
pub const NUM_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // this is the enemy sprite size
pub const ENEMY_SPAWN_TIME: f32 = 5.0;
pub const NUM_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0; // this is the star sprite size
pub const STAR_SPAWN_TIME: f32 = 1.0;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<Score>()
    .init_resource::<HighScores>()
    .init_resource::<StarSpawnTimer>()
    .init_resource::<EnemySpawnTimer>()
    .add_event::<GameOver>()
    .add_systems(Startup,
                 (spawn_player, 
                    spawn_camera, 
                    spawn_enemies,
                    spawn_stars)
                )
    .add_systems(Update,
                 ((player_movement, 
                    confine_player_movement).chain(),
                    (enemy_movement, 
                    update_enemy_direction,
                    confine_enemy_movement).chain(),
                    enemy_hit_player,
                    player_hit_star,
                    update_score,
                    tick_star_spawn_timer,
                    spawn_stars_over_time,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                    exit_game,
                    handle_game_over,
                    update_high_scores,
                    high_scores_updated)
            )
    .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,

}

#[derive(Component)]
pub struct Star {}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> Self {
        HighScores {
            scores: Vec::new(),
        }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME,
                                 TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME,
                                 TimerMode::Repeating),
        }
    }
}

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
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

    (0..NUM_OF_ENEMIES).for_each(|_| {
        enemy_spawn(window, &mut commands, &asset_server);
    });
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    (0..NUM_OF_STARS).into_iter().for_each(|_| {
        star_spawn(window, &mut commands, &asset_server);
    });
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

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    
    }
}

pub fn tick_star_spawn_timer(
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    time: Res<Time>
) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        star_spawn(window, &mut commands, &asset_server);
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

fn star_spawn(
    window: &Window, 
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>
) {
    let random_x = random::<f32>() * window.width();
    let random_y = random::<f32>() * window.height();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(random_x, random_y, 0.0),
            texture: asset_server.load("sprites/star.png"),
            ..default()
        },
        Star {}
    ));
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

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    game_over_event_reader.read().for_each(|event| {
        println!("Game Over! Score: {}", event.score);
    })
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>
) {
    game_over_event_reader.read().for_each(|event| {
        high_scores.scores.push(("Player".to_string(), event.score));
    })
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}