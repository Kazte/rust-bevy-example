use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    core::events::GameOverEvent,
    enemy::components::Enemy,
    score::resources::Score,
    star::{components::Star, systems::STAR_RADIUS},
};

use super::components::Player;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_RADIUS: f32 = 24.0 / 2.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/characters/tile_0000.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length() > 0.0 {
            transform.translation += direction.normalize() * PLAYER_SPEED * time.delta_seconds();
        }
    }
}

pub fn confine_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let x_min = 0.0 + PLAYER_RADIUS;
        let x_max = window.width() - PLAYER_RADIUS;
        let y_min = 0.0 + PLAYER_RADIUS;
        let y_max = window.height() - PLAYER_RADIUS;

        let mut translation = transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the players y position.
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            if distance < PLAYER_RADIUS + STAR_RADIUS {
                println!("Player hit a star!");
                commands.entity(star_entity).despawn();
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pepSound2.ogg"),
                    ..default()
                });

                score.value += 1;
            }
        }
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = 24.0 / 2.0;
            let enemy_radius = 24.0 / 2.0;

            if distance < player_radius + enemy_radius {
                commands.entity(player_entity).despawn();
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/explosionCrunch_000.ogg"),
                    ..default()
                });
                game_over_event_writer.send(GameOverEvent { score: score.value });
            }
        }
    }
}
