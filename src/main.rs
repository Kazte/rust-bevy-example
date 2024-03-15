use bevy::prelude::*;
pub mod components;
pub mod events;
pub mod resources;
mod systems;

use events::*;
use resources::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "My First Bevy Game".to_string(),
                resolution: (800.0, 600.0).into(),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                resizable: false,
                ..Default::default()
            }),
            ..default()
        }))
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<HighScore>()
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<GameOver>()
        .add_event::<GameOverEvent>()
        .add_systems(
            Startup,
            (spawn_camera, spawn_player, spawn_enemies, spawn_stars),
        )
        .add_systems(
            Update,
            (
                restart_game,
                exit_game,
                handle_game_over,
                player_movement,
                confine_player,
                enemies_movement,
                update_enemy_direction,
                enemy_hit_player,
                player_hit_star,
                update_score,
                tick_spawn_timer_star,
                spawn_star_overtime,
                tick_spawn_timer_enemy,
                spawn_enemy_overtime,
                update_highscore,
                higscores_updated,
            ),
        )
        .run();
}
