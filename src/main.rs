use bevy::prelude::*;

mod core;
mod enemy;
mod player;
mod score;
mod star;

use core::CorePlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

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
            CorePlugin,
            EnemyPlugin,
            PlayerPlugin,
            ScorePlugin,
            StarPlugin,
        ))
        .run();
}
