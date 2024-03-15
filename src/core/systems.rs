use bevy::app::AppExit;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::score::resources::HighScore;

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

pub fn update_highscore(
    mut game_over_event_reader: EventReader<GameOverEvent>,
    mut high_scores: ResMut<HighScore>,
) {
    for event in game_over_event_reader.read() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

pub fn higscores_updated(high_scores: Res<HighScore>) {
    if high_scores.is_changed() {
        println!("Highscores: {:?}", high_scores.scores);
    }
}
