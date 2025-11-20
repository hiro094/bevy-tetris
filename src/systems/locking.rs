use bevy::prelude::*;
use crate::components::{Active, GridPosition};
use crate::resources::{ActivePiece, Grid};

#[derive(Message)]
pub struct PieceLocks;

pub fn lock_piece_system(
    mut commands: Commands,
    mut events: MessageReader<PieceLocks>,
    mut grid: ResMut<Grid>,
    active_piece: Option<ResMut<ActivePiece>>,
    mut query: Query<(Entity, &GridPosition, &mut Sprite), With<Active>>,
    mut hold_piece: ResMut<crate::resources::HoldPiece>,
    mut sound_events: MessageWriter<crate::systems::audio::SoundEvent>,
) {
    for _ in events.read() {
        // Remove ActivePiece resource
        if active_piece.is_none() {
            continue; 
        }
        commands.remove_resource::<ActivePiece>();
        
        // Reset hold status
        hold_piece.can_hold = true;
        
        sound_events.write(crate::systems::audio::SoundEvent::Lock);

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
