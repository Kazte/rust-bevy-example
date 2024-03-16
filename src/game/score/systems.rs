use ::bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::resources::Score;

pub const TITLE: &str = "My First Bevy Game";

pub fn update_score(score: Res<Score>, mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    if score.is_changed() {
        let mut window = window_query.get_single_mut().unwrap();

        window.title = format!("{} - Score: {}", TITLE, score.value.to_string());

        println!("Score: {}", score.value.to_string());
    }
}
