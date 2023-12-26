
use bevy::prelude::*;

use super::components::*;
use crate::game::{
    score::resources::Score, 
    enemy::components::Enemy
};

const BACKGROUND_COLOR: Color = Color::rgba(0.5, 0.5, 0.5, 0.25);

pub fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,

                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        },
        HUD {}
    ))
    .with_children(|parent| {
        // === Score ===
        parent.spawn(
            NodeBundle {
                style: Style {
                    width: Val::Px(128.0),
                    height: Val::Px(64.0),
                    flex_direction: FlexDirection::Row,
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        left: Val::Px(16.0),
                        top: Val::Px(4.0),
                        ..default()
                    },

                    ..default()
                },
                background_color: BACKGROUND_COLOR.into(),
                ..default()
            }
        )
        .with_children(|parent| {
            // === Image ===
            parent.spawn(
                ImageBundle {
                    style: Style {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        margin: UiRect::all(Val::Px(4.0)),
                        
                        ..default()
                    },
                    image: asset_server.load("sprites/star.png").into(),
                    ..default()
                }
            );
            // === Text ===
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                " : ",
                                get_text_style(&asset_server)
                            )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },
                ScoreDisplay {}
            ));
        });
        // === Enemy Counter ===
        parent.spawn(
            NodeBundle {
                style: Style {
                    width: Val::Px(128.0),
                    height: Val::Px(64.0),
                    flex_direction: FlexDirection::Row,
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        right: Val::Px(16.0),
                        top: Val::Px(4.0),
                        ..default()
                    },

                    ..default()
                },
                background_color: BACKGROUND_COLOR.into(),
                
                ..default()
            }
        )
        .with_children(|parent| {
            // === Image ===
            parent.spawn(
                ImageBundle {
                    style: Style {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        margin: UiRect::all(Val::Px(4.0)),
                        
                        ..default()
                    },
                    image: asset_server
                    .load("sprites/ball_red_large.png").into(),
                    
                    ..default()
                }
            );
            // === Text ===
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                " : ",
                                get_text_style(&asset_server)
                            )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },
                EnemyCounter {}
            ));
        });
    });
}

pub fn despawn_hud(
    mut commands: Commands,
    hud_query: Query<Entity, With<HUD>>,
) {
    if let Ok(hud_entity) = hud_query.get_single() {
            commands.entity(hud_entity).despawn_recursive();
    }
}

pub fn update_score_display(
    score: Res<Score>,
    mut text_query: Query<&mut Text, With<ScoreDisplay>>
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        text.sections[0].value = format!(" : {}", score.value);
    }
}

pub fn update_enemy_counter(
    mut text_query: Query<&mut Text, With<EnemyCounter>>,
    enemy_query: Query<With<Enemy>>
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        let enemy_count = enemy_query.iter().count();
        text.sections[0].value = format!(" : {}", enemy_count);
    }
}

fn get_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}