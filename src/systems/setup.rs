use bevy::prelude::*;
use crate::resources::{GRID_WIDTH, GRID_HEIGHT};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn setup_grid(mut commands: Commands) {
    // Board background
    commands.spawn((
        Sprite {
            color: Color::srgb(0.1, 0.1, 0.1),
            custom_size: Some(Vec2::new(
                GRID_WIDTH as f32 * crate::resources::CELL_SIZE,
                GRID_HEIGHT as f32 * crate::resources::CELL_SIZE,
            )),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0), // Behind blocks
    ));
}
