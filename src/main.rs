use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use resources::{Grid, GameScore, GameTimer};
use systems::setup::{setup_camera, setup_grid};
use systems::spawning::spawn_tetromino;
use systems::rendering::render_blocks;
use systems::movement::{handle_input, apply_gravity};
use systems::locking::{lock_piece_system, PieceLocks};
use systems::line_clearing::clear_lines;

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
        .init_resource::<Grid>()
        .init_resource::<GameScore>()
        .insert_resource(GameTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .add_message::<PieceLocks>()
        .add_systems(Startup, (setup_camera, setup_grid))
        .add_systems(Update, (
            spawn_tetromino,
            handle_input,
            apply_gravity,
            render_blocks,
            lock_piece_system,
            clear_lines.after(lock_piece_system),
        ))
        .run();
}
