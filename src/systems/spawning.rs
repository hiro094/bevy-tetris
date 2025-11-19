use bevy::prelude::*;
use crate::components::{Active, Block, GridPosition, TetrominoType};
use crate::resources::{ActivePiece, CELL_SIZE, GRID_WIDTH, GRID_HEIGHT};

pub fn spawn_tetromino(
    mut commands: Commands,
    active_pieces: Query<&Active>,
) {
    if !active_pieces.is_empty() {
        return;
    }

    let piece_type = TetrominoType::T; // Fixed for now
    let spawn_position = IVec2::new(GRID_WIDTH / 2, GRID_HEIGHT - 2); // Spawn slightly below top

    let offsets = piece_type.get_offsets();
    let color = piece_type.get_color();

    for (x, y) in offsets.iter() {
        commands.spawn((
            Block,
            Active,
            GridPosition {
                x: spawn_position.x + x,
                y: spawn_position.y + y,
            },
            Sprite {
                color,
                custom_size: Some(Vec2::new(CELL_SIZE - 2.0, CELL_SIZE - 2.0)), // Slightly smaller for gap
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }

    commands.insert_resource(ActivePiece {
        piece_type,
        position: spawn_position,
        rotation_state: 0,
    });
}
