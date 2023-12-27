
mod components;
mod systems;

use bevy::prelude::*;

use systems::*;
use crate::AppState;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (
                spawn_game_over_menu,
                (
                    interact_with_new_game_button,
                    interact_with_main_menu_button,
                    interact_with_quit_button
                )
                .run_if(in_state(AppState::GameOver))
        ))
        .add_systems(OnExit(AppState::GameOver),
                    despawn_game_over_menu);
    }

}