
use bevy::prelude::*;


pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, main_menue);
    }
}

pub fn main_menue() {
    println!("Hello Main Menu");
}