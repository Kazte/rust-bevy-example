use bevy::prelude::*;

pub mod core;
pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
pub mod systems;

use core::CorePlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_plugins((
                CorePlugin,
                StarPlugin,
                PlayerPlugin,
                EnemyPlugin,
                ScorePlugin,
            ))
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::InGame)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
