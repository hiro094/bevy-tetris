use bevy::prelude::*;
use crate::components::{Active, Block, GridPosition, TetrominoType};
use crate::resources::{ActivePiece, CELL_SIZE, GRID_WIDTH, GRID_HEIGHT, NextPiece, Grid};
use crate::GameState;
use crate::systems::collision::is_valid_position;

pub fn setup_next_piece(mut commands: Commands) {
    commands.insert_resource(NextPiece {
        piece_type: get_random_piece(),
    });
}

fn get_random_piece() -> TetrominoType {
    let pieces = [
        TetrominoType::I,
        TetrominoType::J,
        TetrominoType::L,
        TetrominoType::O,
        TetrominoType::S,
        TetrominoType::T,
        TetrominoType::Z,
    ];
    let mut rng = rand::rng();
    use rand::seq::IndexedRandom;
    *pieces.choose(&mut rng).unwrap()
}

pub fn spawn_tetromino(
    mut commands: Commands,
    active_pieces: Query<&Active>,
    mut next_piece: ResMut<NextPiece>,
    grid: Res<Grid>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !active_pieces.is_empty() {
        return;
    }

    let piece_type = next_piece.piece_type;
    next_piece.piece_type = get_random_piece();

    let spawn_position = IVec2::new(GRID_WIDTH / 2, GRID_HEIGHT - 2); // Spawn slightly below top
    let rotation_state = 0;
    let offsets = piece_type.get_offsets_for_rotation(rotation_state);
    let color = piece_type.get_color();

    // Check for Game Over (collision at spawn)
    let mut valid_spawn = true;
    for (x, y) in offsets.iter() {
        let pos_x = spawn_position.x + x;
        let pos_y = spawn_position.y + y;
        if !is_valid_position(&grid, pos_x, pos_y) {
            valid_spawn = false;
            break;
        }
    }

    if !valid_spawn {
        next_state.set(GameState::GameOver);
        return;
    }

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
                custom_size: Some(Vec2::new(CELL_SIZE - 2.0, CELL_SIZE - 2.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));
    }

    commands.insert_resource(ActivePiece {
        piece_type,
        position: spawn_position,
        rotation_state,
    });
}
