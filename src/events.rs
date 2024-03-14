use bevy::prelude::*;

#[derive(Event)]
pub struct GameOverEvent {
    pub score: u32,
}
