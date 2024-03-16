use bevy::prelude::*;

use super::SimulationState;

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.get() {
            SimulationState::Paused => {
                commands.insert_resource(NextState(Some(SimulationState::Running)));
                println!("SimulationState::Paused");
            }
            SimulationState::Running => {
                commands.insert_resource(NextState(Some(SimulationState::Paused)));
                println!("SimulationState::Running");
            }
        }
    }
}
