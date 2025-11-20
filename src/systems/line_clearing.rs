use bevy::prelude::*;
use crate::resources::{Grid, GRID_WIDTH, GRID_HEIGHT, GameScore, GameTimer};
use crate::components::{GridPosition, Block};

pub fn clear_lines(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    mut query: Query<(Entity, &mut GridPosition), With<Block>>,
    mut score: ResMut<GameScore>,
    mut timer: ResMut<GameTimer>,
    mut sound_events: MessageWriter<crate::systems::audio::SoundEvent>,
) {
    let mut lines_cleared = 0;
    let mut y = 0;
    
    while y < GRID_HEIGHT {
        let mut row_full = true;
        for x in 0..GRID_WIDTH {
            if grid.cells[y as usize][x as usize].is_none() {
                row_full = false;
                break;
            }
        }
        
        if row_full {
            lines_cleared += 1;
            
            // Remove blocks in this row
            let mut entities_to_remove = Vec::new();
            for (entity, pos) in query.iter() {
                if pos.y == y {
                    entities_to_remove.push(entity);
                }
            }
            
            for entity in entities_to_remove {
                commands.entity(entity).despawn();
            }
            
            // Shift grid down
            for row in y..(GRID_HEIGHT - 1) {
                grid.cells[row as usize] = grid.cells[(row + 1) as usize];
            }
            grid.cells[(GRID_HEIGHT - 1) as usize] = [None; GRID_WIDTH as usize];
            
            // Shift block entities down
            for (_, mut pos) in query.iter_mut() {
                if pos.y > y {
                    pos.y -= 1;
                }
            }
            
            // Don't increment y, check this row index again (since rows shifted down)
        } else {
            y += 1;
        }
    }
    
    if lines_cleared > 0 {
        // Update score
        score.score += 100 * (1 << (lines_cleared - 1));
        score.lines_cleared += lines_cleared as u32;
        
        // Level up every 10 lines
        let new_level = (score.lines_cleared / 10) + 1;
        if new_level > score.level {
            score.level = new_level;
            // Speed up
            let new_duration = (0.5 * (0.8f32).powi(score.level as i32 - 1)).max(0.05);
            timer.0.set_duration(std::time::Duration::from_secs_f32(new_duration));
        }
        println!("Score: {}, Level: {}", score.score, score.level);
        
        sound_events.write(crate::systems::audio::SoundEvent::Clear);
    }
}
