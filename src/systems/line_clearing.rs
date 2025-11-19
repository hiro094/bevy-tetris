use bevy::prelude::*;
use crate::resources::{Grid, GRID_WIDTH, GRID_HEIGHT, GameScore};
use crate::components::{GridPosition, Block};

pub fn clear_lines(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    mut query: Query<(Entity, &mut GridPosition), With<Block>>,
    mut score: ResMut<GameScore>,
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
            // We need to find entities at this row.
            // This is inefficient with query iteration, but fine for Tetris scale.
            // A better way would be to store Entity in Grid, but we stored Color.
            // So we iterate all blocks.
            
            // We can't remove entities while iterating query if we also want to modify others?
            // We can collect entities to remove.
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
        // Simple scoring: 100 * 2^(lines - 1)
        score.0 += 100 * (1 << (lines_cleared - 1));
        println!("Score: {}", score.0);
    }
}
