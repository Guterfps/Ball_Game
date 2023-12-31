
use bevy::prelude::*;

use crate::game::SimulationState;

pub fn pause_simulation(
    mut simulation_next_state: ResMut<NextState<SimulationState>>
) {
    simulation_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(
    mut simulation_next_state: ResMut<NextState<SimulationState>>
) {
    simulation_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    simulation_next_state: ResMut<NextState<SimulationState>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let state = simulation_state.get();
        if *state == SimulationState::Running {
            pause_simulation(simulation_next_state);
            println!("Paused");
        }
        else if *state == SimulationState::Paused {
            resume_simulation(simulation_next_state);
            println!("Running");
        }
    }
}