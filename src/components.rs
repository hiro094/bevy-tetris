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
    pub fn get_offsets(&self) -> [(i32, i32); 4] {
        match self {
            TetrominoType::I => [(0, 0), (-1, 0), (1, 0), (2, 0)],
            TetrominoType::J => [(0, 0), (-1, 0), (1, 0), (-1, 1)],
            TetrominoType::L => [(0, 0), (-1, 0), (1, 0), (1, 1)],
            TetrominoType::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            TetrominoType::S => [(0, 0), (-1, 0), (0, 1), (1, 1)],
            TetrominoType::T => [(0, 0), (-1, 0), (1, 0), (0, 1)],
            TetrominoType::Z => [(0, 0), (1, 0), (0, 1), (-1, 1)],
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
}
