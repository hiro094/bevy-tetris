use bevy::prelude::*;
use crate::resources::{ActivePiece, Grid, CELL_SIZE, GRID_HEIGHT, GRID_WIDTH};

use crate::systems::collision::is_valid_position;

#[derive(Component)]
pub struct GhostBlock;

pub fn render_ghost(
    mut commands: Commands,
    active_piece: Option<Res<ActivePiece>>,
    grid: Res<Grid>,
    ghost_query: Query<Entity, With<GhostBlock>>,
) {
    // Despawn existing ghost blocks
    for entity in ghost_query.iter() {
        commands.entity(entity).despawn();
    }

    let Some(piece) = active_piece else { return };

    // Calculate ghost position
    let mut ghost_position = piece.position;
    
    // Drop until collision
    loop {
        let next_y = ghost_position.y - 1;
        let mut can_move = true;
        
        let offsets = piece.piece_type.get_offsets_for_rotation(piece.rotation_state);
        
        for (x, y) in offsets.iter() {
            if !is_valid_position(&grid, ghost_position.x + x, next_y + y) {
                can_move = false;
                break;
            }
        }
        
        if can_move {
            ghost_position.y = next_y;
        } else {
            break;
        }
    }
    
    // Don't render if ghost is at the same position as the active piece (optional, but looks cleaner)
    // if ghost_position == piece.position { return; }

    // Spawn ghost blocks
    let offsets = piece.piece_type.get_offsets_for_rotation(piece.rotation_state);
    let color = piece.piece_type.get_color().with_alpha(0.1); // Transparent

    for (x, y) in offsets.iter() {
        let pos_x = ghost_position.x + x;
        let pos_y = ghost_position.y + y;
        
        // Convert to screen coordinates (copied from rendering.rs logic)
        // We should probably refactor coordinate conversion to a helper.
        let screen_x = (pos_x as f32 * CELL_SIZE) - (GRID_WIDTH as f32 * CELL_SIZE / 2.0) + (CELL_SIZE / 2.0);
        let screen_y = (pos_y as f32 * CELL_SIZE) - (GRID_HEIGHT as f32 * CELL_SIZE / 2.0) + (CELL_SIZE / 2.0);

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(CELL_SIZE - 2.0, CELL_SIZE - 2.0)),
                ..default()
            },
            Transform::from_xyz(screen_x, screen_y, 0.5), // Z-index 0.5 to be behind active piece (1.0) but in front of grid (0.0)
            GhostBlock,
        ));
    }
}
