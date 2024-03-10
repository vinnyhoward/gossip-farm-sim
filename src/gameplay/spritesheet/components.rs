use crate::prelude::*;

#[derive(Resource, Debug)]
pub struct PlayerSpriteSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct PurpleCowSpriteSheet(pub Handle<TextureAtlas>);

// Spritesheets by Layer
#[derive(Resource, Debug)]
pub struct ForestDetailsV2Sheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct RampsSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct AppleTreeSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct RegularTreeSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct WoodenHouseSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct WaterObjectsSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct BushesSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct FencesSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct ForestDetailsSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct DarkerGrassPatchSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct DarkGrassPatchSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct SoilSpriteSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct DarkGrassHillsSpriteSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct DarkGrassWaterSpriteSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct WaterSpriteSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct ChattingIconSheet(pub Handle<TextureAtlas>);

#[derive(Resource, Debug)]
pub struct EmotionIconSheet(pub Handle<TextureAtlas>);

pub enum SpriteSheetResource {
    Player(PlayerSpriteSheet),

    // Spritesheets by Layer
    ForestDetailsV2(ForestDetailsV2Sheet),   // 15th Layer
    Ramps(RampsSheet),                       // 14th Layer
    AppleTree(AppleTreeSheet),               // 13th Layer
    RegularTree(RegularTreeSheet),           // 12th Layer
    WoodenHouse(WoodenHouseSheet),           // 11th Layer
    WaterObjects(WaterObjectsSheet),         // 10th Layer
    Bushes(BushesSheet),                     // 9th Layer
    Fences(FencesSheet),                     // 8th Layer
    ForestDetails(ForestDetailsSheet),       // 7th Layer
    DarkerGrassPatch(DarkerGrassPatchSheet), // 6th Layer
    GrassPatch(DarkGrassPatchSheet),         // 5th layer
    Soil(SoilSpriteSheet),                   // 4rd Layer
    SecondGround(DarkGrassHillsSpriteSheet), // 3rd Layer
    Ground(DarkGrassWaterSpriteSheet),       // 2nd Layer
    Water(WaterSpriteSheet),                 // 1st Layer
    PurpleCow(PurpleCowSpriteSheet),
    ChattingIcon(ChattingIconSheet),
    EmotionIcon(EmotionIconSheet),
}

pub struct SpriteSheetInfo {
    pub url: &'static str,
    pub sprite_size: Vec2,
    pub columns: usize,
    pub rows: usize,
    pub resource_type: fn(Handle<TextureAtlas>) -> SpriteSheetResource,
}
