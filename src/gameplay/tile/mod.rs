use crate::prelude::*;

pub mod systems;
use systems::{spawn_map_from_csv, spawn_water, water_sprite_animation};

pub mod components;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((spawn_water, spawn_map_from_csv))
            .add_system(water_sprite_animation);
    }
}
