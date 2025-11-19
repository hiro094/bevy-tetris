use bevy::prelude::*;
use crate::components::{Active, GridPosition};
use crate::resources::{ActivePiece, GameTimer, Grid};
use crate::systems::collision::is_valid_position;

pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut active_piece: Option<ResMut<ActivePiece>>,
    grid: Res<Grid>,
    mut query: Query<(Entity, &mut GridPosition), With<Active>>,
) {
    let Some(mut piece) = active_piece else { return };

    let mut delta = IVec2::ZERO;
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        delta.x -= 1;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        delta.x += 1;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        delta.y -= 1;
    }

    if delta != IVec2::ZERO {
        let new_pivot = piece.position + delta;
        // Check if all current blocks can move by delta.
        let mut can_move = true;
        for (_, pos) in query.iter() {
            if !is_valid_position(&grid, pos.x + delta.x, pos.y + delta.y) {
                can_move = false;
                break;
            }
        }

        if can_move {
            piece.position += delta;
            for (_, mut pos) in query.iter_mut() {
                pos.x += delta.x;
                pos.y += delta.y;
            }
        }
    }
    
    // Rotation (Up Arrow)
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        // Simple rotation: 90 degrees clockwise
        // (x, y) -> (-y, x) relative to pivot
        
        let pivot = piece.position;
        let mut new_positions = Vec::new();
        let mut can_rotate = true;
        
        // Collect current entities and their potential new positions
        let mut entities = Vec::new();
        for (entity, pos) in query.iter() {
            let rel_x = pos.x - pivot.x;
            let rel_y = pos.y - pivot.y;
            
            // Clockwise: (x, y) -> (y, -x)
            let new_rel_x = rel_y;
            let new_rel_y = -rel_x;
            
            let new_x = pivot.x + new_rel_x;
            let new_y = pivot.y + new_rel_y;
            
            if !is_valid_position(&grid, new_x, new_y) {
                can_rotate = false;
                break;
            }
            new_positions.push(GridPosition { x: new_x, y: new_y });
            entities.push(entity);
        }
        
        if can_rotate {
            for (i, entity) in entities.iter().enumerate() {
                if let Ok((_, mut pos)) = query.get_mut(*entity) {
                    *pos = new_positions[i];
                }
            }
            piece.rotation_state = (piece.rotation_state + 1) % 4;
        }
    }
}

pub fn apply_gravity(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut active_piece: Option<ResMut<ActivePiece>>,
    grid: Res<Grid>,
    mut query: Query<&mut GridPosition, With<Active>>,
    mut lock_events: MessageWriter<crate::systems::locking::PieceLocks>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    
    let Some(mut piece) = active_piece else { return };
    
    let delta = IVec2::new(0, -1);
    
    let mut can_move = true;
    for pos in query.iter() {
        if !is_valid_position(&grid, pos.x + delta.x, pos.y + delta.y) {
            can_move = false;
            break;
        }
    }

    if can_move {
        piece.position += delta;
        for mut pos in query.iter_mut() {
            pos.x += delta.x;
            pos.y += delta.y;
        }
    } else {
        lock_events.write(crate::systems::locking::PieceLocks);
    }
}
