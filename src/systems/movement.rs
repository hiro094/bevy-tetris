use bevy::prelude::*;
use crate::components::{Active, GridPosition};
use crate::resources::{ActivePiece, GameTimer, Grid};
use crate::systems::collision::is_valid_position;

pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    active_piece: Option<ResMut<ActivePiece>>,
    grid: Res<Grid>,
    mut query: Query<(Entity, &mut GridPosition), With<Active>>,
    mut hold_piece: ResMut<crate::resources::HoldPiece>,
    mut commands: Commands,
    mut sound_events: MessageWriter<crate::systems::audio::SoundEvent>,
) {
    let Some(mut piece) = active_piece else { return };

    // Hold (C or Shift)
    if keyboard_input.just_pressed(KeyCode::KeyC) || keyboard_input.just_pressed(KeyCode::ShiftLeft) || keyboard_input.just_pressed(KeyCode::ShiftRight) {
        if hold_piece.can_hold {
            // ... (existing hold logic)
            // Despawn current active blocks
            for (entity, _) in query.iter() {
                commands.entity(entity).despawn();
            }
            
            let current_type = piece.piece_type;
            
            // Swap or spawn
            if let Some(held_type) = hold_piece.piece_type {
                // Swap
                hold_piece.piece_type = Some(current_type);
                spawn_piece(&mut commands, held_type);
            } else {
                // First hold
                hold_piece.piece_type = Some(current_type);
                commands.remove_resource::<ActivePiece>();
            }
            
            hold_piece.can_hold = false;
            return; 
        }
    }

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
            sound_events.write(crate::systems::audio::SoundEvent::Move);
        }
    }
    
    // Rotation (Up Arrow)
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        let old_rot = piece.rotation_state;
        let new_rot = (old_rot + 1) % 4;
        
        let new_offsets = piece.piece_type.get_offsets_for_rotation(new_rot);
        let kicks = piece.piece_type.get_srs_kicks(old_rot, new_rot);
        
        let mut successful_kick = None;
        let mut final_new_positions = Vec::new();
        
        // Try each kick
        for kick in kicks {
            let test_pivot = piece.position + IVec2::new(kick.0, kick.1);
            let mut kick_valid = true;
            let mut kick_positions = Vec::new();
            
            for (x, y) in new_offsets.iter() {
                let pos_x = test_pivot.x + x;
                let pos_y = test_pivot.y + y;
                
                if !is_valid_position(&grid, pos_x, pos_y) {
                    kick_valid = false;
                    break;
                }
                kick_positions.push(GridPosition { x: pos_x, y: pos_y });
            }
            
            if kick_valid {
                successful_kick = Some(test_pivot);
                final_new_positions = kick_positions;
                break;
            }
        }
        
        if let Some(new_pivot) = successful_kick {
            // Apply rotation
            piece.position = new_pivot;
            piece.rotation_state = new_rot;
            
            let mut i = 0;
            for (_, mut pos) in query.iter_mut() {
                if i < final_new_positions.len() {
                    *pos = final_new_positions[i];
                    i += 1;
                }
            }
            sound_events.write(crate::systems::audio::SoundEvent::Rotate);
        }
    }
}

pub fn apply_gravity(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    active_piece: Option<ResMut<ActivePiece>>,
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

fn spawn_piece(commands: &mut Commands, piece_type: crate::components::TetrominoType) {
    use crate::resources::{CELL_SIZE, GRID_WIDTH, GRID_HEIGHT, ActivePiece};
    use crate::components::{Block, Active, GridPosition};
    
    let spawn_position = IVec2::new(GRID_WIDTH / 2, GRID_HEIGHT - 2);
    let offsets = piece_type.get_offsets_for_rotation(0);
    let color = piece_type.get_color();
    
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
        rotation_state: 0,
    });
}
