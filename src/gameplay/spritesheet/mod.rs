use crate::prelude::*;

pub mod systems;
use systems::load_spritesheets;

pub mod components;

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_spritesheets.in_base_set(StartupSet::PreStartup));
    }
}
