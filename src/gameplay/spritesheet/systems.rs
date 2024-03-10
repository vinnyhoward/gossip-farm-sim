use crate::gameplay::spritesheet::components::{
    AppleTreeSheet, BushesSheet, ChattingIconSheet, DarkGrassHillsSpriteSheet, DarkGrassPatchSheet,
    DarkGrassWaterSpriteSheet, DarkerGrassPatchSheet, EmotionIconSheet, FencesSheet,
    ForestDetailsSheet, ForestDetailsV2Sheet, PlayerSpriteSheet, PurpleCowSpriteSheet, RampsSheet,
    RegularTreeSheet, SoilSpriteSheet, SpriteSheetInfo, SpriteSheetResource, WaterObjectsSheet,
    WaterSpriteSheet, WoodenHouseSheet,
};
use crate::gameplay::tile::components::WaterSpriteAnimation;
use crate::prelude::*;

const SPRITE_SHEETS: &[SpriteSheetInfo] = &[
    SpriteSheetInfo {
        url: "spritesheets/spritesheet.png",
        sprite_size: Vec2::new(48.0, 48.0),
        columns: 8,
        rows: 24,
        resource_type: |handle| SpriteSheetResource::Player(PlayerSpriteSheet(handle)),
    },
    SpriteSheetInfo {
        url: "spritesheets/purple_cow.png",
        sprite_size: Vec2::new(32.0, 32.0),
        columns: 8,
        rows: 9,
        resource_type: |handle| SpriteSheetResource::PurpleCow(PurpleCowSpriteSheet(handle)),
    },
    SpriteSheetInfo {
        url: "spritesheets/forest_details_v2.png",
        sprite_size: Vec2::new(16.0, 16.0),
        columns: 12,
        rows: 7,
        resource_type: |handle| SpriteSheetResource::ForestDetailsV2(ForestDetailsV2Sheet(handle)),
    },
    SpriteSheetInfo {
        url: "spritesheets/dark_hills_tall.png",
        sprite_size: Vec2::new(16.0, 16.0),
        columns: 6,
        rows: 8,
        resource_type: |handle| SpriteSheetResource::Ramps(RampsSheet(handle)),
    },
    SpriteSheetInfo {
        url: "spritesheets/apple_tree.png",
        sprite_size: Vec2::new(16.0, 16.0),
        columns: 36,
        rows: 15,
        resource_type: |handle| SpriteSheetResource::AppleTree(AppleTreeSheet(handle)),
    },
    SpriteSheetInfo {
        url: "spritesheets/regular_tree.png",
        sprite_size: Vec2::new(16.0, 16.0),
        columns: 36,
        rows: 15,
        resource_type: |handle| SpriteSheetResource::RegularTree(RegularTreeSheet(handle)),
    },
    SpriteSheetInfo {
        url: "spritesheets/wooden_house.png",
        sprite_size: Vec2::new(16.0, 16.0),
        columns: 6,
        rows: 5,
        resource_type: |handle| SpriteSheetResource::WoodenHouse(WoodenHouseSheet(handle)),
    },
    SpriteSheetInfo {
        url: "spritesheets/water_objects.png",
        sprite_size: Vec2::new(16.0, 16.0),
        columns: 12,
        rows: 2,
        resource_type: |handle| SpriteSheetResource::WaterObjects(WaterObjectsSheet(handle)),
    },
    SpriteSheetInfo {
        url: "spritesheets/bushes.png",
        sprite_size: Vec2::new(16.0, 16.0),
        columns: 11,
        rows: 12,
        resource_type: |handle| SpriteSheetResource::Bushes(BushesSheet(handle)),
    },
    SpriteSheetInfo {
        url: "spritesheets/fences.png",
        sprite_size: Vec2::new(16.0, 16.0),
        columns: 8,
        rows: 4,
        resource_type: |handle| SpriteSheetResource::Fences(FencesSheet(handle)),
    },
    SpriteSheetInfo {
        url: "spritesheets/forest_details.png",
        sprite_size: Vec2::new(16.0, 16.0),
        columns: 12,
        rows: 5,
        resource_type: |handle| SpriteSheetResource::ForestDetails(ForestDetailsSheet(handle)),
    },
    SpriteSheetInfo {
        url: "spritesheets/darker_grass_patch.png",
        sprite_size: Vec2::new(16.0, 16.0),
        columns: 11,
        rows: 7,
        resource_type: |handle| {
            SpriteSheetResource::DarkerGrassPatch(DarkerGrassPatchSheet(handle))
        },
    },
    SpriteSheetInfo {
        url: "spritesheets/dark_grass_patch.png",
        sprite_size: Vec2::new(16.0, 16.0),
        columns: 11,
        rows: 7,
        resource_type: |handle| SpriteSheetResource::GrassPatch(DarkGrassPatchSheet(handle)),
    },
    SpriteSheetInfo {
        url: "spritesheets/soil.png",
        sprite_size: Vec2::new(16.0, 16.0),
        columns: 11,
        rows: 7,
        resource_type: |handle| SpriteSheetResource::Soil(SoilSpriteSheet(handle)),
    },
    SpriteSheetInfo {
        url: "spritesheets/dark_grass_hills.png",
        sprite_size: Vec2::new(16.0, 16.0),
        columns: 11,
        rows: 7,
        resource_type: |handle| {
            SpriteSheetResource::SecondGround(DarkGrassHillsSpriteSheet(handle))
        },
    },
    SpriteSheetInfo {
        url: "spritesheets/dark_grass_water.png",
        sprite_size: Vec2::new(16.0, 16.0),
        columns: 9,
        rows: 8,
        resource_type: |handle| SpriteSheetResource::Ground(DarkGrassWaterSpriteSheet(handle)),
    },
    SpriteSheetInfo {
        url: "spritesheets/water.png",
        sprite_size: Vec2::new(16.0, 16.0),
        columns: 4,
        rows: 1,
        resource_type: |handle| SpriteSheetResource::Water(WaterSpriteSheet(handle)),
    },
    SpriteSheetInfo {
        url: "icons/chatting_icon.png",
        sprite_size: Vec2::new(48.0, 48.0),
        columns: 4,
        rows: 3,
        resource_type: |handle| SpriteSheetResource::ChattingIcon(ChattingIconSheet(handle)),
    },
    SpriteSheetInfo {
        url: "icons/emotion_icons.png",
        sprite_size: Vec2::new(24.0, 24.0),
        columns: 6,
        rows: 1,
        resource_type: |handle| SpriteSheetResource::EmotionIcon(EmotionIconSheet(handle)),
    },
];

pub fn load_spritesheets(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for info in SPRITE_SHEETS {
        let texture_handle = assets.load(info.url);
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            info.sprite_size,
            info.columns,
            info.rows,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let resource = (info.resource_type)(texture_atlas_handle);
        match resource {
            SpriteSheetResource::Player(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::PurpleCow(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::ForestDetailsV2(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::Ramps(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::AppleTree(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::RegularTree(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::WoodenHouse(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::WaterObjects(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::Bushes(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::Fences(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::ForestDetails(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::DarkerGrassPatch(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::GrassPatch(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::Soil(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::SecondGround(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::Ground(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::Water(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::ChattingIcon(res) => {
                commands.insert_resource(res);
            }
            SpriteSheetResource::EmotionIcon(res) => {
                commands.insert_resource(res);
            }
        }
    }
}

pub fn spawn_player_spritesheet_sprite(
    commands: &mut Commands,
    player_spritesheet: &PlayerSpriteSheet,
    index: usize,
    translation: Vec3,
) -> Entity {
    let sprite = TextureAtlas::new(index);

    commands
        .spawn(SpriteSheetBundle {
            atlas: player_spritesheet.0.clone(),
            sprite,
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

pub fn spawn_cow_spritesheet_sprite(
    commands: &mut Commands,
    purple_cow_spritesheet: &PurpleCowSpriteSheet,
    index: usize,
    translation: Vec3,
) -> Entity {
    let sprite = TextureAtlas::new(index);

    commands
        .spawn(SpriteSheetBundle {
            atlas: purple_cow_spritesheet.0.clone(),
            sprite,
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

pub fn spawn_map(
    commands: &mut Commands,
    spritesheet: &TextureAtlas,
    index: usize,
    translation: Vec3,
) -> Entity {
    let sprite = TextureAtlas::new(index);

    commands
        .spawn(SpriteSheetBundle {
            atlas: spritesheet.clone(),
            sprite,
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

pub fn spawn_water_spritesheet_sprite(
    commands: &mut Commands,
    water_spritesheet: &WaterSpriteSheet,
    index: usize,
    translation: Vec3,
) -> Entity {
    let sprite = TextureAtlas::new(index);

    commands
        .spawn(SpriteSheetBundle {
            atlas: water_spritesheet.0.clone(),
            sprite,
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(WaterSpriteAnimation {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            frame_count: 4,
            current_frame: 0,
            base_index: 0,
        })
        .id()
}

pub fn spawn_chatting_sprite(
    commands: &mut Commands,
    chatting_icon: &ChattingIconSheet,
    index: usize,
    translation: Vec3,
) -> Entity {
    let sprite = TextureAtlas::new(index);

    commands
        .spawn(SpriteSheetBundle {
            atlas: chatting_icon.0.clone(),
            sprite,
            transform: Transform {
                translation,
                scale: Vec3::new(0.65, 0.65, 0.65),
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}

pub fn spawn_emote_sprite(
    commands: &mut Commands,
    emotion_icon: &EmotionIconSheet,
    index: usize,
    translation: Vec3,
) -> Entity {
    let sprite = TextureAtlas::new(index);

    commands
        .spawn(SpriteSheetBundle {
            atlas: emotion_icon.0.clone(),
            sprite,
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..Default::default()
        })
        .id()
}
