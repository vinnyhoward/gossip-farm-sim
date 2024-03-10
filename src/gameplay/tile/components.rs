use crate::prelude::*;
use bevy::prelude::{Component, Entity};

// Define your Map component
#[derive(Component, Default, Debug)]
pub struct TiledMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Entity>,
}

#[derive(Component, Debug)]
pub struct BasicTileCollider;

#[derive(Component, Debug)]
pub struct BasicPassiveTileCollider;

#[derive(Component, Default, Debug)]
pub struct WaterSpriteAnimation {
    pub timer: Timer,
    pub frame_count: usize,
    pub current_frame: usize,
    pub base_index: usize,
}

pub struct CSVMapData {
    pub csv_file_path: &'static str,
    pub spritesheet_resource_name: &'static str,
    pub collision_indices: Vec<i32>,
    pub passive_collision_indices: Vec<i32>,
    pub z_index: f32,
    pub is_vertically_inverted: bool,
}

pub struct NpcSpawnData {
    pub csv_file_path: &'static str,
    pub z_index: f32,
}

pub struct PlayerSpawnData {
    pub csv_file_path: &'static str,
    pub z_index: f32,
}
