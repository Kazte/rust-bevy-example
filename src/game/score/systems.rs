use ::bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::core::events::GameOverEvent;

use super::{resources::Score, HighScore};

pub const TITLE: &str = "My First Bevy Game";

pub fn update_score(score: Res<Score>, mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if score.is_changed() {
        let mut window = window_query.get_single_mut().unwrap();

        window.title = format!("{} - Score: {}", TITLE, score.value.to_string());

        println!("Score: {}", score.value.to_string());
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
