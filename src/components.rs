use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub struct Active;

#[derive(Component)]
pub struct Tetromino {
    pub piece_type: TetrominoType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TetrominoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl TetrominoType {
    // SRS Offsets
    // I: 4x4, others 3x3, O 2x2 (conceptually)
    // We will use a local coordinate system where (0,0) is the center of rotation.
    // For I, center is between blocks. For others, center is the center block.
    // Actually, standard SRS uses a grid and rotates around a pivot.
    // Let's define offsets relative to the pivot.
    
    pub fn get_offsets(&self) -> [(i32, i32); 4] {
        match self {
            // I: Pivot is at (0.5, 0.5) in 4x4 grid?
            // Let's use standard integer offsets relative to a "center" block.
            // I (Horizontal): (-1, 0), (0, 0), (1, 0), (2, 0) -> Pivot is (0.5, 0.5)?
            // SRS I spawn:
            // . . . .
            // # # # #
            // . . . .
            // . . . .
            // If pivot is (0,0) of the grid?
            // Let's stick to the current implementation for now but ensure they are correct relative to a pivot.
            // Current: I => [(0, 0), (-1, 0), (1, 0), (2, 0)] -> Pivot (0,0) is the second block.
            // SRS I rotates around the center of the 4x4 grid.
            // This requires float coordinates or a different grid system.
            // Alternative: Use a lookup table for all 4 rotations for each piece.
            // This is easier and more robust for SRS.
            
            TetrominoType::I => [(0, 0), (-1, 0), (1, 0), (2, 0)], // Placeholder, will be replaced by rotation system
            TetrominoType::J => [(0, 0), (-1, 0), (1, 0), (-1, 1)],
            TetrominoType::L => [(0, 0), (-1, 0), (1, 0), (1, 1)],
            TetrominoType::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            TetrominoType::S => [(0, 0), (-1, 0), (0, 1), (1, 1)],
            TetrominoType::T => [(0, 0), (-1, 0), (1, 0), (0, 1)],
            TetrominoType::Z => [(0, 0), (1, 0), (0, 1), (-1, 1)],
        }
    }

    // Helper to get offsets for a specific rotation state (0, 1, 2, 3)
    pub fn get_offsets_for_rotation(&self, rotation: usize) -> [(i32, i32); 4] {
        // SRS definitions
        // We can define the positions of the 4 blocks for each rotation state.
        // Coordinates are relative to the pivot.
        
        match self {
            TetrominoType::I => match rotation % 4 {
                0 => [(-1, 0), (0, 0), (1, 0), (2, 0)],
                1 => [(1, 1), (1, 0), (1, -1), (1, -2)],
                2 => [(-1, -1), (0, -1), (1, -1), (2, -1)],
                3 => [(0, 1), (0, 0), (0, -1), (0, -2)],
                _ => unreachable!(),
            },
            TetrominoType::J => match rotation % 4 {
                0 => [(-1, 1), (-1, 0), (0, 0), (1, 0)],
                1 => [(1, 1), (0, 1), (0, 0), (0, -1)],
                2 => [(-1, 0), (0, 0), (1, 0), (1, -1)],
                3 => [(0, 1), (0, 0), (0, -1), (-1, -1)],
                _ => unreachable!(),
            },
            TetrominoType::L => match rotation % 4 {
                0 => [(-1, 0), (0, 0), (1, 0), (1, 1)],
                1 => [(0, 1), (0, 0), (0, -1), (1, -1)],
                2 => [(-1, -1), (-1, 0), (0, 0), (1, 0)],
                3 => [(-1, 1), (0, 1), (0, 0), (0, -1)],
                _ => unreachable!(),
            },
            TetrominoType::O => match rotation % 4 {
                _ => [(0, 0), (1, 0), (0, 1), (1, 1)], // O doesn't rotate
            },
            TetrominoType::S => match rotation % 4 {
                0 => [(-1, 0), (0, 0), (0, 1), (1, 1)],
                1 => [(0, 1), (0, 0), (1, 0), (1, -1)],
                2 => [(-1, -1), (0, -1), (0, 0), (1, 0)],
                3 => [(-1, 1), (-1, 0), (0, 0), (0, -1)],
                _ => unreachable!(),
            },
            TetrominoType::T => match rotation % 4 {
                0 => [(-1, 0), (0, 0), (1, 0), (0, 1)],
                1 => [(0, 1), (0, 0), (0, -1), (1, 0)],
                2 => [(-1, 0), (0, 0), (1, 0), (0, -1)],
                3 => [(0, 1), (0, 0), (0, -1), (-1, 0)],
                _ => unreachable!(),
            },
            TetrominoType::Z => match rotation % 4 {
                0 => [(-1, 1), (0, 1), (0, 0), (1, 0)],
                1 => [(1, 1), (1, 0), (0, 0), (0, -1)],
                2 => [(-1, 0), (0, 0), (0, -1), (1, -1)],
                3 => [(0, 1), (0, 0), (-1, 0), (-1, -1)],
                _ => unreachable!(),
            },
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            TetrominoType::I => Color::srgb(0.0, 1.0, 1.0), // Cyan
            TetrominoType::J => Color::srgb(0.0, 0.0, 1.0), // Blue
            TetrominoType::L => Color::srgb(1.0, 0.5, 0.0), // Orange
            TetrominoType::O => Color::srgb(1.0, 1.0, 0.0), // Yellow
            TetrominoType::S => Color::srgb(0.0, 1.0, 0.0), // Green
            TetrominoType::T => Color::srgb(0.5, 0.0, 0.5), // Purple
            TetrominoType::Z => Color::srgb(1.0, 0.0, 0.0), // Red
        }
    }

    pub fn get_srs_kicks(&self, old_rot: usize, new_rot: usize) -> Vec<(i32, i32)> {
        let k = (old_rot % 4, new_rot % 4);
        match self {
            TetrominoType::O => vec![(0, 0)],
            TetrominoType::I => match k {
                (0, 1) => vec![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                (1, 0) => vec![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                (1, 2) => vec![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                (2, 1) => vec![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                (2, 3) => vec![(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
                (3, 2) => vec![(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
                (3, 0) => vec![(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
                (0, 3) => vec![(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
                _ => vec![(0, 0)],
            },
            _ => match k {
                (0, 1) => vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                (1, 0) => vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                (1, 2) => vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
                (2, 1) => vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
                (2, 3) => vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                (3, 2) => vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                (3, 0) => vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
                (0, 3) => vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
                _ => vec![(0, 0)],
            },
        }
    }
}
