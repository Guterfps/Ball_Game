
use bevy::prelude::*;

use super::components::*;
use crate::{
    main_menu::styles::*, 
    game::SimulationState,
    AppState
};

pub fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        (NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(50.0),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                column_gap: Val::Px(8.0),
                ..default()
            },
            background_color: Color::rgba(0.2, 0.2, 0.2, 0.7).into(),
            border_color: Color::BLACK.into(),
            ..default()
        },
        PauseMenu {},
    ))
    .with_children(|parent| {
        // === Title ===
        parent.spawn(
            NodeBundle {
                style: TITLE_STYLE,
                ..default()        
            }
        ).with_children(|parent| {
            // Text
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Pause Menu",
                                get_title_text_style(&asset_server)
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        });
        // === Resume Button ===
        parent.spawn((
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: TRANSPERNT_BUTTON_COLOR.into(),
                ..default()
            },
            ResumeButton {}
        ))
        .with_children(|parent| {
            parent.spawn(
            TextBundle {
                text: Text {
                    sections: vec![
                        TextSection::new(
                            "Resume", 
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
                background_color: TRANSPERNT_BUTTON_COLOR.into(),
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
    });
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(entity) = pause_menu_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn interact_with_resume_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), 
                        (Changed<Interaction>, With<ResumeButton>)>,
    mut next_game_state: ResMut<NextState<SimulationState>>
) {
    if let Ok((interaction, mut background_color)) = 
            button_query.get_single_mut() {
                *background_color = match interaction {
                        Interaction::Pressed => {
                            next_game_state.set(SimulationState::Running);
                            PRESSED_BUTTON_COLOR.into()
                        },
                        Interaction::Hovered => HOVERED_BUTTON_COLOR.into(),
                        Interaction::None => TRANSPERNT_BUTTON_COLOR.into()
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