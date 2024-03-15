use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct GameOver {
    pub value: bool,
}

impl Default for GameOver {
    fn default() -> Self {
        GameOver { value: false }
    }
}
