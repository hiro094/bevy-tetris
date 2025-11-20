use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use resources::{Grid, GameScore, GameTimer};
use systems::setup::{setup_camera, setup_grid};
use systems::spawning::{spawn_tetromino, setup_next_piece};
use systems::rendering::render_blocks;
use systems::movement::{handle_input, apply_gravity};
use systems::locking::{lock_piece_system, PieceLocks};
use systems::line_clearing::clear_lines;
use systems::ui::{setup_ui, update_score, update_next_piece, update_hold_piece};
use systems::game_over::{game_over_setup, restart_game, cleanup_game_over};
use systems::ghost::render_ghost;
use systems::audio::{setup_audio, play_sound_system, SoundEvent};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Tetris".into(),
                resolution: (800, 600).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .init_resource::<Grid>()
        .init_resource::<GameScore>()
        .init_resource::<resources::HoldPiece>()
        .insert_resource(GameTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .add_message::<PieceLocks>()
        .add_message::<SoundEvent>()
        .add_systems(Startup, (setup_camera, setup_grid, setup_next_piece, setup_ui, setup_audio))
        .add_systems(Update, (
            spawn_tetromino,
            handle_input,
            apply_gravity,
            lock_piece_system,
            clear_lines,
            render_blocks,
            update_score,
            update_next_piece,
            update_hold_piece,
            render_ghost,
            play_sound_system,
        ).run_if(in_state(GameState::Playing)))
        .add_systems(OnEnter(GameState::GameOver), game_over_setup)
        .add_systems(Update, restart_game.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
        .run();
}
