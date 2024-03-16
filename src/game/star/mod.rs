pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            // .add_systems(Startup, spawn_stars)
            .add_systems(OnEnter(AppState::InGame), spawn_stars)
            .add_systems(OnExit(AppState::InGame), despawn_stars)
            .add_systems(
                Update,
                (tick_spawn_timer_star, spawn_star_overtime)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
