use bevy::app::AppExit;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::score::resources::HighScore;
use crate::game::SimulationState;
use crate::AppState;

use super::{GameOver, GameOverEvent};

pub fn restart_game(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        println!("Restarting game!");
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.5),
        ..default()
    });
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOverEvent>,
    mut game_over: ResMut<GameOver>,
) {
    for event in game_over_event_reader.read() {
        println!("Game Over! Score: {}", event.score);
        game_over.value = true;
    }
}

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        let current_state = app_state.get().clone();
        if current_state != AppState::InGame {
            commands.insert_resource(NextState(Some(AppState::InGame)));
            println!("Transitioning to InGame state")
        }
    }
}

pub fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        let current_state = app_state.get().clone();
        if current_state != AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Transitioning to MainMenu state")
        }
    }
}
