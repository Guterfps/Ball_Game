
use bevy::prelude::*;

mod enemy;
mod player;
mod score;
mod star;
mod systems;
mod pause_menu;
mod game_over_menu;
mod hud;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use pause_menu::PauseMenuPlugin;
use game_over_menu::GameOverMenuPlugin;
use hud::HUDPlugin;
use crate::events::GameOver;
use crate::AppState;

use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<SimulationState>()
        .add_event::<GameOver>()
        .add_plugins((
            EnemyPlugin, 
            PlayerPlugin, 
            ScorePlugin, 
            StarPlugin,
            PauseMenuPlugin,
            GameOverMenuPlugin,
            HUDPlugin,
        ))
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