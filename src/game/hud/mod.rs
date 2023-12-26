
mod components;
mod systems;

use bevy::prelude::*;

use systems::*;

use crate::AppState;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Game),
            spawn_hud)
        .add_systems(Update, (
                    update_score_display,
                    update_enemy_counter
                )
                .run_if(in_state(AppState::Game))
        )
        .add_systems(OnExit(AppState::Game), 
            despawn_hud);
    }
}