use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

mod game;
mod main_menu;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
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
        }),))
        .init_state::<AppState>()
        .add_plugins((GamePlugin, MainMenuPlugin))
        .run();
}
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}
