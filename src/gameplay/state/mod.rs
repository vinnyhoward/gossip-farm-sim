use crate::prelude::*;

mod systems;
use systems::{transition_to_game_state, transition_to_main_menu_state};

pub mod components;
use components::{CameraState, GameState, HttpRequestState};
pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_state::<CameraState>()
            .add_state::<HttpRequestState>()
            .add_systems((transition_to_game_state, transition_to_main_menu_state));
    }
}
