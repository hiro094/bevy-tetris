use bevy::prelude::*;
use crate::components::GridPosition;
use crate::resources::{BOARD_ORIGIN_X, BOARD_ORIGIN_Y, CELL_SIZE};

pub fn render_blocks(
    mut query: Query<(&GridPosition, &mut Transform)>,
) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation.x = BOARD_ORIGIN_X + pos.x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
        transform.translation.y = BOARD_ORIGIN_Y + pos.y as f32 * CELL_SIZE + CELL_SIZE / 2.0;
    }
}
