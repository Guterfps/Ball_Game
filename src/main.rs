
use bevy::prelude::*;

pub mod events;
mod systems;
mod game;
mod main_menu;

use systems::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, GamePlugin, MainMenuPlugin))
    .add_state::<AppState>()
    .add_systems(Startup,spawn_camera)
    .add_systems(Update,(
                exit_game,
                handle_game_over,
                transition_to_game_state,
                transition_to_main_menu_state,
    ))
    .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
