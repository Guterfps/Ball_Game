
use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;
use crate::AppState;
use crate::game::SimulationState;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // this is the player sprite size

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
    Movment,
    Confinement,
    PlayerSpawn
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .configure_sets(Update, 
            PlayerSystemSet::Movment
                    .before(PlayerSystemSet::Confinement))
        .configure_sets(OnEnter(AppState::Game), 
                        PlayerSystemSet::PlayerSpawn)
        .add_systems(OnEnter(AppState::Game),
                spawn_player.in_set(PlayerSystemSet::PlayerSpawn))
        .add_systems(Update,(
                player_movement.in_set(PlayerSystemSet::Movment), 
                confine_player_movement.in_set(PlayerSystemSet::Confinement),
                enemy_hit_player,
                player_hit_star
            )
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
        )
        .add_systems(OnExit(AppState::Game), 
                despawn_player);
    }
}
