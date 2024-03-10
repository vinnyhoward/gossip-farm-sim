use super::components::{CameraState, GameState};
use crate::prelude::*;

pub fn _pause_game(mut game_state_next_state: ResMut<NextState<GameState>>) {
    game_state_next_state.set(GameState::Pause);
}

pub fn _resume_game(mut game_state_next_state: ResMut<NextState<GameState>>) {
    game_state_next_state.set(GameState::Playing);
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<GameState>>,
    mut app_state_next_state: ResMut<NextState<GameState>>,
    mut camera_state_next_state: ResMut<NextState<CameraState>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        if app_state.current() != GameState::MainMenu {
            camera_state_next_state.set(CameraState::MainMenuCamera);
            app_state_next_state.set(GameState::MainMenu);
        }
    }
}

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<GameState>>,
    mut app_state_next_state: ResMut<NextState<GameState>>,
    mut camera_state_next_state: ResMut<NextState<CameraState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        if app_state.current() != GameState::Playing {
            camera_state_next_state.set(CameraState::ManualCameraControl);
            app_state_next_state.set(GameState::Playing);
        }
    }
}
