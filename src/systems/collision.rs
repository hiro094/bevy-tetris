use crate::resources::{Grid, GRID_WIDTH, GRID_HEIGHT};

pub fn is_valid_position(
    grid: &Grid,
    x: i32,
    y: i32,
) -> bool {
    if x < 0 || x >= GRID_WIDTH || y < 0 || y >= GRID_HEIGHT {
        return false;
    }
    
    if grid.cells[y as usize][x as usize].is_some() {
        return false;
    }
    
    true
}
