use crate::prelude::*;

use super::state::components::CameraState;

mod systems;
use systems::{
    chatting_icon_on_hover, chatting_icon_spawner, chatting_icon_tick, emote_event,
    emote_icon_tick, emote_movement_system, inactive_player_proximity_detection,
    inactive_player_proximity_detection_event, inactive_player_roaming_system,
    pair_entity_convergence_system, player_activation_system, player_confinement_system,
    player_input, player_movement_system, player_spawner, player_sprite_animation,
    queue_pair_entity_conversation_request, tick_player_interaction_timers,
    update_app_state_system,
};

pub mod components;
use components::EntityPairConversationState;

pub mod events;
use events::EmoteEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EntityPairConversationState::default())
            .add_event::<EmoteEvent>()
            .add_system(emote_event)
            .add_startup_system(player_spawner)
            .add_system(inactive_player_proximity_detection)
            .add_systems((
                player_sprite_animation,
                pair_entity_convergence_system,
                tick_player_interaction_timers,
                player_confinement_system,
                player_movement_system,
                player_activation_system,
                update_app_state_system,
                inactive_player_proximity_detection_event,
                inactive_player_roaming_system,
                chatting_icon_on_hover,
                chatting_icon_spawner,
                chatting_icon_tick,
                queue_pair_entity_conversation_request,
                emote_movement_system,
            ))
            .add_system(emote_icon_tick)
            .add_system(player_input.run_if(in_state(CameraState::FollowPlayer)));
    }
}
