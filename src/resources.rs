use bevy::prelude::*;
use crate::components::TetrominoType;

pub const GRID_WIDTH: i32 = 10;
pub const GRID_HEIGHT: i32 = 20;
pub const CELL_SIZE: f32 = 30.0;
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

// Calculate board dimensions
pub const BOARD_WIDTH_PX: f32 = GRID_WIDTH as f32 * CELL_SIZE;
pub const BOARD_HEIGHT_PX: f32 = GRID_HEIGHT as f32 * CELL_SIZE;

// Center the board
pub const BOARD_ORIGIN_X: f32 = -BOARD_WIDTH_PX / 2.0;
pub const BOARD_ORIGIN_Y: f32 = -BOARD_HEIGHT_PX / 2.0;

#[derive(Resource)]
pub struct Grid {
    // 2D array representing the grid. 
    // Stores the color of the block if occupied, or None if empty.
    pub cells: [[Option<Color>; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            cells: [[None; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
        }
    }
}

#[derive(Resource)]
pub struct ActivePiece {
    pub piece_type: TetrominoType,
    pub position: IVec2, // Grid coordinates
    pub rotation_state: usize, // 0, 1, 2, 3
}

#[derive(Resource, Default)]
pub struct GameScore(pub u32);

#[derive(Resource)]
pub struct GameTimer(pub Timer);
