// packages
use crate::helpers::bindgen;
use bevy::window::PresentMode;
use serde::{Deserialize, Serialize};
mod prelude {
    pub use bevy::prelude::TimerMode;
    pub use bevy::prelude::*;
    pub use bevy::sprite::TextureAtlas;
    pub use bevy::window::PrimaryWindow;
    pub use bevy_inspector_egui::prelude::*;
    pub use rand::prelude::*;
    pub use serde;
    pub use serde_wasm_bindgen;
    pub use std::collections::HashMap;
    pub use std::path::PathBuf;
    pub use wasm_bindgen::prelude::*;
    pub use wasm_bindgen::JsValue;
    pub const SCREEN_WIDTH: f32 = 1600.0;
    pub const SCREEN_HEIGHT: f32 = 960.0;
    pub const PLAYER_SIZE: f32 = 16.0;
    // pub use pecs::prelude::*;
    pub const ENTITY_CONVO_DISTANCE: f32 = 15.0;
    pub const ENTITY_CONVO_DURATION: f32 = 10.0;
    pub const TILE_SIZE: f32 = 16.;
    pub const CAMERA_SCALE_FACTOR: f32 = 0.3;
    pub const API_URL: &'static str = "http://localhost:7070";
    pub use crate::bindgen::*;
    // pub use crate::CryptoCreature;
}

// modules
use prelude::*;
mod data;
mod gameplay;
mod helpers;
use gameplay::state::StatesPlugin;
use crate::gameplay::GamePlayPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "EtherPets".into(),
                        resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                        present_mode: PresentMode::AutoVsync,
                        canvas: Some("#wasm-canvas".to_string()),
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin { ..default() }),
        )
        .add_plugin(StatesPlugin)
        .add_plugin(GamePlayPlugin)
        .run();
}
