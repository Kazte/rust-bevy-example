use bevy::prelude::*;

pub mod events;
pub mod resources;
mod systems;

use events::*;
use resources::*;
use systems::*;
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameOver>()
            .add_event::<GameOverEvent>()
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (
                    restart_game,
                    exit_game,
                    handle_game_over,
                    update_highscore,
                    higscores_updated,
                ),
            );
    }
}
