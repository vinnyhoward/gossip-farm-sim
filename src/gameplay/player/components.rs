use crate::prelude::*;
use bevy_inspector_egui::prelude::*;

#[derive(Component, Reflect, Resource, Default, InspectorOptions, Debug)]
#[reflect(Resource, InspectorOptions)]
pub struct EtherPet;

#[derive(Component, Debug)]
pub struct ActiveEtherPet;

#[derive(Debug, Clone)]
pub enum Quadrants {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    None,
}

#[derive(Resource, Debug, Clone)]
pub struct PairEntityConversationData {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub entity_a_pos: Vec3,
    pub entity_b_pos: Vec3,
    pub entity_a_id: String,
    pub entity_b_id: String,
    pub quadrant: Quadrants,
    pub chatting_icon_spawned: bool,
    pub conversation_http_request_sent: bool,
}

#[derive(Default, Resource, Debug)]
pub struct EntityPairConversationState {
    pub pair_entities_in_conversation: HashMap<(Entity, Entity), PairEntityConversationData>,
    pub entities_in_conversation: HashMap<Entity, bool>,
}

#[derive(Component, Debug)]
pub enum BasicEmotions {
    Happiness,
    Excitement,
    Sadness,
    Fear,
    Disgust,
    Hate,
}

#[derive(Component, Debug)]
pub struct PlayerData {
    pub name: String,
    pub attack_dmg: i32,
    pub speed: f32,
    pub player_id: String,
    pub emotion: BasicEmotions,
}

#[derive(Component, Debug)]
pub struct PlayerUniqueId {
    pub uuid: String,
}

#[derive(Component, Debug)]
pub struct EmoteParentEntity {
    pub entity: Entity,
}
