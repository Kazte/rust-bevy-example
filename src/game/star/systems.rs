use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::core::resources::GameOver;

use super::{components::Star, resources::StarSpawnTimer};

pub const NUMBER_OF_STARS: u8 = 10;
pub const STAR_RADIUS: f32 = 22.0 / 2.0;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let random_x = rand::random::<f32>() * window.width();
        let random_y = rand::random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/characters/tile_0008.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn despawn_stars(mut commands: Commands, query: Query<Entity, With<Star>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn tick_spawn_timer_star(
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    time: Res<Time>,
    game_over: ResMut<GameOver>,
) {
    if game_over.value == false {
        star_spawn_timer.timer.tick(time.delta());
    }
}

pub fn spawn_star_overtime(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = rand::random::<f32>() * window.width();
        let random_y = rand::random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/characters/tile_0008.png"),
                ..default()
            },
            Star {},
        ));
    }
}
