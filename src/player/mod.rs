
use bevy::prelude::*;

mod components;
mod systems;

use systems::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // this is the player sprite size

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movment,
    Confinement
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .configure_sets(Update, 
            PlayerSystemSet::Movment
                    .before(PlayerSystemSet::Confinement))
        .add_systems(Startup, spawn_player)
        .add_systems(Update,(
                player_movement.in_set(PlayerSystemSet::Movment), 
                confine_player_movement.in_set(PlayerSystemSet::Confinement),
                enemy_hit_player,
                player_hit_star
        ));
    }
}
