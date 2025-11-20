use bevy::prelude::*;
use crate::resources::{GameScore, NextPiece};

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct NextPieceDisplay;

#[derive(Component)]
pub struct HoldPieceDisplay;

pub fn setup_ui(mut commands: Commands) {
    // Score
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ScoreText,
    ));

    // Next Piece Label
    commands.spawn((
        Text::new("Next:"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            right: Val::Px(150.0),
            ..default()
        },
    ));
    
    // Next Piece Container
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(90.0),
            right: Val::Px(50.0),
            width: Val::Px(100.0),
            height: Val::Px(100.0),
            ..default()
        },
        NextPieceDisplay,
    ));
    // Hold Piece Label
    commands.spawn((
        Text::new("Hold:"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(150.0),
            ..default()
        },
    ));
    
    // Hold Piece Container
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(90.0),
            left: Val::Px(150.0),
            width: Val::Px(100.0),
            height: Val::Px(100.0),
            ..default()
        },
        HoldPieceDisplay,
    ));
}

pub fn update_score(score: Res<GameScore>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in query.iter_mut() {
            *text = Text::new(format!("Score: {}", score.score));
        }
    }
}

pub fn update_next_piece(
    mut commands: Commands,
    next_piece: Res<NextPiece>,
    query: Query<Entity, With<NextPieceDisplay>>,
    children_query: Query<&Children>,
) {
    if next_piece.is_changed() {
        if let Some(container_entity) = query.iter().next() {
            // Despawn existing children
            if let Ok(children) = children_query.get(container_entity) {
                for child in children.iter() {
                    commands.entity(child).despawn();
                }
            }
            
            // Spawn new blocks
            let offsets = next_piece.piece_type.get_offsets(); // Use default rotation
            let color = next_piece.piece_type.get_color();
            
            for (x, y) in offsets.iter() {
                // Map x, y to UI coordinates
                // Center is roughly 50, 50
                let ui_x = 40.0 + (*x as f32 * 20.0);
                let ui_y = 40.0 - (*y as f32 * 20.0); // Invert y for UI
                
                commands.entity(container_entity).with_children(|parent| {
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(ui_x),
                            top: Val::Px(ui_y),
                            width: Val::Px(18.0),
                            height: Val::Px(18.0),
                            ..default()
                        },
                        BackgroundColor(color),
                    ));
                });
            }
        }
    }
}

pub fn update_hold_piece(
    mut commands: Commands,
    hold_piece: Res<crate::resources::HoldPiece>,
    query: Query<Entity, With<HoldPieceDisplay>>,
    children_query: Query<&Children>,
) {
    if hold_piece.is_changed() {
        if let Some(container_entity) = query.iter().next() {
            // Despawn existing children
            if let Ok(children) = children_query.get(container_entity) {
                for child in children.iter() {
                    commands.entity(child).despawn();
                }
            }
            
            if let Some(piece_type) = hold_piece.piece_type {
                // Spawn new blocks
                let offsets = piece_type.get_offsets(); // Use default rotation
                let color = if hold_piece.can_hold { piece_type.get_color() } else { piece_type.get_color().with_alpha(0.5) };
                
                for (x, y) in offsets.iter() {
                    // Map x, y to UI coordinates
                    // Center is roughly 50, 50
                    let ui_x = 40.0 + (*x as f32 * 20.0);
                    let ui_y = 40.0 - (*y as f32 * 20.0); // Invert y for UI
                    
                    commands.entity(container_entity).with_children(|parent| {
                        parent.spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Px(ui_x),
                                top: Val::Px(ui_y),
                                width: Val::Px(18.0),
                                height: Val::Px(18.0),
                                ..default()
                            },
                            BackgroundColor(color),
                        ));
                    });
                }
            }
        }
    }
}
