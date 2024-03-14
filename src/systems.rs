use bevy::app::AppExit;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::*;
use crate::events::*;
use crate::resources::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_RADIUS: f32 = 24.0 / 2.0;
pub const ENEMY_SPEED: f32 = 100.0;
pub const ENEMY_RADIUS: f32 = 22.0 / 2.0;
pub const NUMBER_OF_STARS: u8 = 10;
pub const STAR_RADIUS: f32 = 22.0 / 2.0;

pub fn restart_game(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        println!("Restarting game!");
    }
}

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

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..10 {
        let random_x = rand::random::<f32>() * window.width();
        let random_y = rand::random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/characters/tile_0024.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(rand::random::<f32>(), rand::random::<f32>()),
            },
        ));
    }
}

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

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.5),
        ..default()
    });
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

pub fn enemies_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction.normalize() * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&mut Enemy, &Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let x_min = 0.0 + ENEMY_RADIUS;
    let x_max = window.width() - ENEMY_RADIUS;
    let y_min = 0.0 + ENEMY_RADIUS;
    let y_max = window.height() - ENEMY_RADIUS;

    for (mut enemy, transform) in enemy_query.iter_mut() {
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
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

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn tick_spawn_timer_star(
    mut star_spawn_timer: ResMut<StarSpawnTimer>,
    time: Res<Time>,
    mut game_over_event_reader: EventReader<GameOverEvent>,
) {
    if game_over_event_reader.read().next().is_none() {
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

pub fn tick_spawn_timer_enemy(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    mut game_over_event_reader: EventReader<GameOverEvent>,
) {
    if game_over_event_reader.read().next().is_none() {
        enemy_spawn_timer.timer.tick(time.delta());
    }
}

pub fn spawn_enemy_overtime(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let random_x = rand::random::<f32>() * window.width();
        let random_y = rand::random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/characters/tile_0024.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(rand::random::<f32>(), rand::random::<f32>()),
            },
        ));
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOverEvent>) {
    for event in game_over_event_reader.read() {
        println!("Game Over! Score: {}", event.score);
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
