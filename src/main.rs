use bevy::prelude::*;

pub mod events;
pub mod resources;
mod systems;

mod enemy;
mod player;
mod score;
mod star;

use enemy::EnemyPlugin;
use events::*;
use player::PlayerPlugin;
use resources::*;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "My First Bevy Game".to_string(),
                    resolution: (800.0, 600.0).into(),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    resizable: false,
                    ..Default::default()
                }),
                ..default()
            }),
            EnemyPlugin,
            PlayerPlugin,
            ScorePlugin,
            StarPlugin,
        ))
        .init_resource::<GameOver>()
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
        )
        .run();
}
