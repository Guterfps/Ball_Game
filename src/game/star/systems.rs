
use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::game::star::{
    components::*, 
    resources::*,
    NUM_OF_STARS,
    STAR_SIZE
};
use crate::game::enemy::confine_actor_axis;

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

pub fn despawn_stars(
    mut commands: Commands,
    star_query: Query<Entity, With<Star>>
) {
    star_query.iter().for_each(|star_entity| {
        commands.entity(star_entity).despawn();
    })
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


fn star_spawn(
    window: &Window, 
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>
) {
    let window_with = window.width();
    let window_height = window.height();
    let mut random_x = random::<f32>() * window_with;
    let mut random_y = random::<f32>() * window_height;

    confine_actor_axis(&mut random_x, window_with, STAR_SIZE);
    confine_actor_axis(&mut random_y, window_height, STAR_SIZE);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(random_x, random_y, 0.0),
            texture: asset_server.load("sprites/star.png"),
            ..default()
        },
        Star {}
    ));
}
