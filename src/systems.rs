
use bevy::{prelude::*, window::{PrimaryWindow, WindowResized}, app::AppExit};

use crate::{events::*, AppState};

#[derive(Component)]
pub struct Camera;

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, 
                                        window.height() / 2.0, 0.0),
            ..default()
        },
        Camera {}
    )
    );
}

pub fn move_camera_when_window_resize(
    mut resize_event: EventReader<WindowResized>,
    mut camera_query: Query<&mut Transform, With<Camera>>
) {
    resize_event.read().for_each(|event| {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = event.width / 2.0;
        camera_transform.translation.y = event.height / 2.0;
    });
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if *app_state.get() != AppState::Game {
            next_app_state.set(AppState::Game);
            println!("Transitioning to Game State");
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if *app_state.get() != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            println!("Transitioning to Main Menu State");
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    game_over_event_reader.read().for_each(|event| {
        println!("Game Over! Score: {}", event.score);
        next_app_state.set(AppState::GameOver);
    })
}