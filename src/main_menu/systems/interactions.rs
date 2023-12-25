
use bevy::{prelude::*, app::AppExit};

use crate::{
    main_menu::{styles::*, components::*}, 
    AppState
};

pub fn interact_with_play_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), 
                        (Changed<Interaction>, With<PlayButton>)>,
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