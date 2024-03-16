use bevy::{prelude::*, window::PrimaryWindow};
use rand::{thread_rng, Rng};

use crate::game::{core::resources::GameOver, player::components::Player};

use super::{components::*, resources::EnemySpawnTimer};

pub const ENEMY_SPEED: f32 = 100.0;
pub const ENEMY_RADIUS: f32 = 22.0 / 2.0;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let mut rng = thread_rng();

    for _ in 0..5 {
        let random_x = rand::random::<f32>() * window.width();
        let random_y = rand::random::<f32>() * window.height();
        let mut enemy_type: EnemyTypeEnum = EnemyTypeEnum::Chaser;

        if rand::random::<f32>() > 0.5 {
            enemy_type = EnemyTypeEnum::Wanderer;
        }

        // random direction between -1 and 1

        let direction = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/characters/tile_0024.png"),
                ..default()
            },
            Enemy {
                direction,
                speed: ENEMY_SPEED,
                enemy_type,
            },
        ));
    }
}

pub fn enemies_movement(
    mut set: ParamSet<(
        Query<(&mut Transform, &Enemy), With<Enemy>>,
        Query<&Transform, With<Player>>,
    )>,
    time: Res<Time>,
    game_over: ResMut<GameOver>,
) {
    if game_over.value == false {
        let mut player_position: Vec3 = Vec3::new(0.0, 0.0, 0.0);

        if let Ok(player_transform) = set.p1().get_single() {
            player_position = player_transform.translation;
        }

        for (mut transform, entity) in set.p0().iter_mut() {
            match entity.enemy_type {
                EnemyTypeEnum::Wanderer => {
                    let direction = Vec3::new(entity.direction.x, entity.direction.y, 0.0);
                    transform.translation +=
                        direction.normalize() * ENEMY_SPEED * time.delta_seconds();
                }
                EnemyTypeEnum::Chaser => {
                    let direction = player_position - transform.translation;
                    transform.translation +=
                        direction.normalize() * ENEMY_SPEED * time.delta_seconds();
                }
            }
        }
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

pub fn tick_spawn_timer_enemy(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    game_over: ResMut<GameOver>,
) {
    if game_over.value == false {
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
        let mut enemy_type: EnemyTypeEnum = EnemyTypeEnum::Chaser;

        if rand::random::<f32>() > 0.5 {
            enemy_type = EnemyTypeEnum::Wanderer;
        }

        // random direction between -1 and 1
        let mut rng = thread_rng();
        let direction = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/characters/tile_0024.png"),
                ..default()
            },
            Enemy {
                direction,
                speed: ENEMY_SPEED,
                enemy_type,
            },
        ));
    }
}

pub fn despawn_enemies(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for query in query.iter() {
        commands.entity(query).despawn();
    }
}
