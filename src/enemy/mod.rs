
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

use systems::*;
use resources::*;

pub const NUM_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // this is the enemy sprite size


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<EnemySpawnTimer>()
        .add_systems(Startup, spawn_enemies)
        .add_systems(Update, (
                    (enemy_movement, 
                    update_enemy_direction,
                    confine_enemy_movement).chain(),
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time
        ));
    }
}

pub fn confine_actor_axis(axis: &mut f32, window_size: f32, actor_size: f32) {
    let half_actor_size = actor_size / 2.0;
    let min = half_actor_size;
    let max = window_size - half_actor_size;

    if *axis < min {
        *axis = min;
    } else if *axis > max {
        *axis = max;
    }
}