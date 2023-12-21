
use bevy::prelude::*;

pub mod events;
mod systems;
mod enemy;
mod player;
mod score;
mod star;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;
use events::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_event::<GameOver>()
    .add_plugins((
        EnemyPlugin, 
        PlayerPlugin, 
        ScorePlugin, 
        StarPlugin))
    .add_systems(Startup,spawn_camera)
    .add_systems(Update,(
                exit_game,
                handle_game_over,
    ))
    .run();
}
