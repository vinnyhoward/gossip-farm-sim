use crate::prelude::*;

// pub mod audio;
// pub mod camera;
pub mod components;
// pub mod debug;
pub mod events;
// pub mod npc;
pub mod player;
pub mod spritesheet;
pub mod state;
pub mod systems;
pub mod tile;
// use audio::AudioSystemPlugin;
// use camera::CameraPlugin;
use events::ConversationActionEvent;
// use npc::NpcPlugin;
use player::PlayerPlugin;
use spritesheet::SpriteSheetPlugin;
// use tile::TileMapPlugin;

use systems::exit_game;

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ConversationActionEvent>()
            .add_system(exit_game)
            // .add_plugin(CameraPlugin)
            .add_plugin(PlayerPlugin)
        // .add_plugin(NpcPlugin)
        .add_plugin(SpriteSheetPlugin)
        // .add_plugin(AudioSystemPlugin)
        // .add_plugin(TileMapPlugin);
    }
}
