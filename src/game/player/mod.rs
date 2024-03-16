pub mod components;
pub mod systems;

use bevy::prelude::*;

use systems::*;

use crate::AppState;

use super::SimulationState;

// System set
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, MovementSystemSet.before(ConfinementSystemSet))
            // .add_systems(Startup, spawn_player)
            .add_systems(OnEnter(AppState::InGame), spawn_player)
            .add_systems(OnExit(AppState::InGame), despawn_player)
            .add_systems(
                Update,
                (
                    player_movement.in_set(MovementSystemSet),
                    confine_player.in_set(ConfinementSystemSet),
                    enemy_hit_player,
                    player_hit_star,
                )
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
