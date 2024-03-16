use bevy::prelude::*;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub mod components;
pub mod resources;
pub mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            // .add_systems(Startup, spawn_enemies)
            .add_systems(OnEnter(AppState::InGame), spawn_enemies)
            .add_systems(OnExit(AppState::InGame), despawn_enemies)
            .add_systems(
                Update,
                (
                    enemies_movement,
                    update_enemy_direction,
                    tick_spawn_timer_enemy,
                    spawn_enemy_overtime,
                )
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
