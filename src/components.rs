use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
    pub speed: f32,
    pub enemy_type: EnemyTypeEnum,
}

pub enum EnemyTypeEnum {
    Wanderer,
    Chaser,
}

#[derive(Component)]
pub struct Star {}
