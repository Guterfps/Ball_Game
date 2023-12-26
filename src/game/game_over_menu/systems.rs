
use bevy::{prelude::*, app::AppExit};

use super::components::*;
use crate::{
    main_menu::styles::*, 
    game::score::resources::HighScores,
    AppState,
};

pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    high_scores: Res<HighScores>
) {
    if let Some(score) = high_scores.scores.last() {
        commands.spawn(
            (NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            GameOverMenu {},
        ))
        .with_children(|parent| {
            // === Title ===
            parent.spawn(
                NodeBundle {
                    style: TITLE_STYLE,
                    ..default()        
                }
            ).with_children(|parent| {
                // Image 1
                parent.spawn(
                    ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server
                        .load("sprites/ball_blue_large.png")
                        .into(),
                        ..default()
                    }
                );
                // Text
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    format!("Game Over!\n {} final score: {}", 
                                            score.0, score.1),
                                    get_title_text_style(&asset_server)
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        }
                    );
                    
                    // Image 2
                    parent.spawn(
                        ImageBundle {
                            style: IMAGE_STYLE,
                    image: asset_server
                    .load("sprites/ball_red_large.png")
                    .into(),
                    ..default()
                }
            );
        });
        // === New Game Button ===
        parent.spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            NewGameButton {}
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "New Game", 
                                get_button_text_style(&asset_server)
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        // === Main Menu Button ===
        parent.spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            MainMenuButton {}
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Main Menu", 
                                get_button_text_style(&asset_server)
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                    },
                    ..default()
                });
            });
        // === Quit Button ===
        parent.spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            QuitButton {}
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Quit", 
                                get_button_text_style(&asset_server)
                            )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                });
            });
        });
    }
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(entity) = pause_menu_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}


pub fn interact_with_new_game_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), 
                        (Changed<Interaction>, With<NewGameButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if let Ok((interaction, mut background_color)) = 
                button_query.get_single_mut() {
                    *background_color = match *interaction {
                    Interaction::Pressed => {
                        next_app_state.set(AppState::Game);
                        PRESSED_BUTTON_COLOR.into()
                    },
                    Interaction::Hovered => HOVERED_BUTTON_COLOR.into(),
                    Interaction::None => NORMAL_BUTTON_COLOR.into(),
                }
    }
}

pub fn interact_with_main_menu_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), 
                        (Changed<Interaction>, With<MainMenuButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if let Ok((interaction, mut background_color)) = 
                button_query.get_single_mut() {
                    *background_color = match *interaction {
                    Interaction::Pressed => {
                        next_app_state.set(AppState::MainMenu);
                        PRESSED_BUTTON_COLOR.into()
                    },
                    Interaction::Hovered => HOVERED_BUTTON_COLOR.into(),
                    Interaction::None => TRANSPERNT_BUTTON_COLOR.into(),
                }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_evet_writer: EventWriter<AppExit>,
    mut button_query: Query<(&Interaction, &mut BackgroundColor), 
                        (Changed<Interaction>, With<QuitButton>)>
) {
    if let Ok((interaction, mut background_color)) = 
                button_query.get_single_mut() {
                    *background_color = match *interaction {
                    Interaction::Pressed => {
                        app_exit_evet_writer.send(AppExit);
                        PRESSED_BUTTON_COLOR.into()
                    },
                    Interaction::Hovered => HOVERED_BUTTON_COLOR.into(),
                    Interaction::None => NORMAL_BUTTON_COLOR.into(),
                }
    }
}