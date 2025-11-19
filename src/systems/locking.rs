use bevy::prelude::*;
use crate::components::{Active, GridPosition, Block};
use crate::resources::{ActivePiece, Grid, GameScore};

#[derive(Message)]
pub struct PieceLocks;

pub fn lock_piece_system(
    mut commands: Commands,
    mut events: MessageReader<PieceLocks>,
    mut grid: ResMut<Grid>,
    mut active_piece: Option<ResMut<ActivePiece>>,
    mut query: Query<(Entity, &GridPosition, &mut Sprite), With<Active>>,
    mut score: ResMut<GameScore>,
) {
    for _ in events.read() {
        // Remove ActivePiece resource
        if let Some(piece) = active_piece.as_mut() {
            // We could use piece info if needed
        } else {
            continue; 
        }
        commands.remove_resource::<ActivePiece>();

        // Move blocks to grid
        for (entity, pos, mut sprite) in query.iter_mut() {
            commands.entity(entity).remove::<Active>();
            
            // Update grid
            if pos.y >= 0 && pos.y < crate::resources::GRID_HEIGHT && pos.x >= 0 && pos.x < crate::resources::GRID_WIDTH {
                grid.cells[pos.y as usize][pos.x as usize] = Some(sprite.color);
            }
            
            // Visual feedback?
            sprite.color = sprite.color.with_alpha(0.8); // Slightly darken?
        }
    }
}
