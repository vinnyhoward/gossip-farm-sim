// packages
use bevy::sprite::collide_aabb::collide;
use csv::Reader;
use std::fs::File;
use std::str::FromStr;

use crate::prelude::*;

// components
use crate::gameplay::spritesheet::components::{
    AppleTreeSheet, BushesSheet, DarkGrassHillsSpriteSheet, DarkGrassPatchSheet,
    DarkGrassWaterSpriteSheet, DarkerGrassPatchSheet, FencesSheet, ForestDetailsSheet,
    ForestDetailsV2Sheet, RampsSheet, RegularTreeSheet, SoilSpriteSheet, WaterObjectsSheet,
    WaterSpriteSheet, WoodenHouseSheet,
};
use crate::gameplay::tile::components::{
    BasicPassiveTileCollider, BasicTileCollider, TiledMap, WaterSpriteAnimation,
};

use crate::gameplay::player::components::EtherPet;
// systems
use crate::gameplay::spritesheet::systems::{spawn_map, spawn_water_spritesheet_sprite};

// helpers
use crate::data::csv_map_data::get_csv_map_data;

pub fn spawn_map_from_csv(
    mut commands: Commands,
    // TODO: Ghetto way of grabbing spritesheet handles. Need to find a better way
    layer_1: Res<DarkGrassWaterSpriteSheet>,
    layer_2: Res<DarkGrassHillsSpriteSheet>,
    layer_3: Res<SoilSpriteSheet>,
    layer_4: Res<DarkGrassPatchSheet>,
    layer_5: Res<DarkerGrassPatchSheet>,
    layer_6: Res<ForestDetailsSheet>,
    layer_7: Res<FencesSheet>,
    layer_8: Res<BushesSheet>,
    layer_9: Res<WaterObjectsSheet>,
    layer_10: Res<WoodenHouseSheet>,
    layer_11: Res<RegularTreeSheet>,
    layer_12: Res<AppleTreeSheet>,
    layer_13: Res<RampsSheet>,
    layer_14: Res<ForestDetailsV2Sheet>,
) {
    let csv_map_data = get_csv_map_data();

    for map_data in csv_map_data {
        let csv_map_file = match File::open(map_data.csv_file_path) {
            Ok(file) => file,
            Err(error) => {
                panic!("There was a problem opening the file: {:?}", error);
            }
        };
        let mut reader = Reader::from_reader(csv_map_file);
        let mut tiles = Vec::new();
        let mut records: Vec<_> = reader.records().collect();
        let map_height = records.len();
        let map_width = records[0].as_mut().unwrap().len();

        let half_map_width = (map_width as f32 * TILE_SIZE) / 2.0;
        let half_map_height = (map_height as f32 * TILE_SIZE) / 2.0;

        // TODO: Ghetto way of grabbing spritesheet handles. Need to find a better way
        let spritesheet_handle = match map_data.spritesheet_resource_name {
            "DarkGrassWaterSpriteSheet" => layer_1.0.clone(),
            "DarkGrassHillsSpriteSheet" => layer_2.0.clone(),
            "SoilSpriteSheet" => layer_3.0.clone(),
            "DarkGrassPatchSheet" => layer_4.0.clone(),
            "DarkerGrassPatchSheet" => layer_5.0.clone(),
            "ForestDetailsSheet" => layer_6.0.clone(),
            "FencesSheet" => layer_7.0.clone(),
            "BushesSheet" => layer_8.0.clone(),
            "WaterObjectsSheet" => layer_9.0.clone(),
            "WoodenHouseSheet" => layer_10.0.clone(),
            "RegularTreeSheet" => layer_11.0.clone(),
            "AppleTreeSheet" => layer_12.0.clone(),
            "RampsSheet" => layer_13.0.clone(),
            "ForestDetailsV2Sheet" => layer_14.0.clone(),
            _ => panic!(
                "No spritesheet found for {:?}",
                map_data.spritesheet_resource_name
            ),
        };

        for (y, record_result) in records.iter().enumerate() {
            let record: &csv::StringRecord = record_result.as_ref().unwrap();
            for (x, cell) in record.iter().enumerate() {
                let tile_index = i32::from_str(cell).unwrap();
                let tile_translation = Vec3::new(
                    x as f32 * TILE_SIZE - half_map_width,
                    -(y as f32) * TILE_SIZE + half_map_height,
                    map_data.z_index,
                );

                if tile_index != -1 {
                    let tile = spawn_map(
                        &mut commands,
                        &spritesheet_handle,
                        tile_index as usize,
                        tile_translation,
                    );

                    if map_data.collision_indices.contains(&tile_index) {
                        commands.entity(tile).insert(BasicTileCollider);
                    }

                    if map_data.passive_collision_indices.contains(&tile_index) {
                        commands.entity(tile).insert(BasicPassiveTileCollider);
                    }

                    tiles.push(tile);
                }
            }
        }

        commands
            .spawn_empty()
            .insert(Name::new("Gossip Farm Sim"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .insert(TiledMap {
                width: map_width,
                height: map_height,
                tiles,
            });
    }
}

pub fn spawn_water(mut commands: Commands, water_sheet: Res<WaterSpriteSheet>) {
    let tile_count_x = (SCREEN_WIDTH / TILE_SIZE) as usize;
    let tile_count_y = (SCREEN_HEIGHT / TILE_SIZE) as usize;

    let half_screen_width = SCREEN_WIDTH / 2.0;
    let half_screen_height = SCREEN_HEIGHT / 2.0;

    let mut tiles: Vec<Entity> = Vec::new();

    for y in 0..tile_count_y {
        for x in 0..tile_count_x {
            let tile_index = 0;

            let tile = spawn_water_spritesheet_sprite(
                &mut commands,
                &water_sheet,
                tile_index,
                Vec3::new(
                    x as f32 * TILE_SIZE - half_screen_width,
                    -(y as f32) * TILE_SIZE + half_screen_height,
                    0.0,
                ),
            );

            tiles.push(tile);
        }
    }

    commands
        .spawn_empty()
        .insert(Name::new("Water Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default());
}

pub fn water_sprite_animation(
    mut query: Query<(&mut TextureAtlasSprite, &mut WaterSpriteAnimation)>,
    time: Res<Time>,
) {
    for (mut sprite, mut animation) in query.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.finished() {
            animation.current_frame = (animation.current_frame + 1) % animation.frame_count;
            sprite.index = animation.base_index + animation.current_frame;
            animation.timer.reset();
        }
    }
}

pub fn basic_wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<BasicTileCollider>, Without<EtherPet>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE * 0.01),
            wall_transform.translation,
            Vec2::splat(TILE_SIZE),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}
