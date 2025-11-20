use bevy::prelude::*;
use crate::{GameState, resources::{Grid, GameScore, GRID_HEIGHT, GRID_WIDTH}, components::Block};

#[derive(Component)]
pub struct GameOverUI;

pub fn game_over_setup(
    mut commands: Commands,
    mut sound_events: MessageWriter<crate::systems::audio::SoundEvent>,
) {
    sound_events.write(crate::systems::audio::SoundEvent::GameOver);
    
    commands.spawn((
        Text::new("GAME OVER\nPress R to Restart"),
        TextFont {
            font_size: 60.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(40.0),
            left: Val::Percent(20.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        GameOverUI,
    ));
}

pub fn restart_game(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut grid: ResMut<Grid>,
    mut score: ResMut<GameScore>,
    blocks: Query<Entity, With<Block>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        // Reset Grid
        grid.cells = [[None; GRID_WIDTH as usize]; GRID_HEIGHT as usize];
        
        // Reset Score
        score.score = 0;
        score.level = 1;
        score.lines_cleared = 0;
        
        // Despawn all blocks
        for entity in blocks.iter() {
            commands.entity(entity).despawn();
        }
        
        next_state.set(GameState::Playing);
    }
}

pub fn cleanup_game_over(
    mut commands: Commands,
    ui: Query<Entity, With<GameOverUI>>,
) {
    for entity in ui.iter() {
        commands.entity(entity).despawn();
    }
}
