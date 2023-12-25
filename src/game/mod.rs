
use bevy::prelude::*;

mod enemy;
mod player;
mod score;
mod star;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use crate::events::GameOver;
use crate::AppState;

use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<SimulationState>()
        .add_event::<GameOver>()
        .add_systems(OnEnter(AppState::Game), 
                    pause_simulation)
        .add_plugins((
            EnemyPlugin, 
            PlayerPlugin, 
            ScorePlugin, 
            StarPlugin))
        .add_systems(Update,
        toggle_simulation.run_if(in_state(AppState::Game)))
        .add_systems(OnExit(AppState::Game),
                    resume_simulation);    
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}