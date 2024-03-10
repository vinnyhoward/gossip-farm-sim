use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct EntityActionState {
    pub action: GenericActions,
}

#[derive(Component, Debug)]
pub struct Clickable;

#[derive(Component, Debug)]
pub struct AnimationTimers {
    pub idle_timer: Timer,
    pub walk_timer: Timer,
    pub attack_timer: Timer,
    pub eat_timer: Timer,
    pub emote_timer: Timer,
}

#[derive(Component, Debug, Clone)]
pub struct Animation {
    pub last_direction: DirectionIntent,
    pub current_frame: usize,
    pub frame_count: usize,
}

#[derive(Component, Debug)]
pub struct Attack {
    pub attack_active: bool,
    pub attack_dmg: i32,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Debug)]
pub struct Movement {
    pub direction: Vec2,
    pub speed: f32,
}

#[derive(Component, Debug)]
pub struct Roaming {
    // walking
    pub roam_direction: Vec3,
    pub roam_time: f32,
    pub roam_max_time: f32,
    pub roam_speed: f32,
    // idle
    pub idle_time: f32,
    pub idle_max_time: f32,
    pub is_idle: bool,
    // eating
    pub eating_time: f32,
    pub eating_max_time: f32,
    pub is_eating: bool,
    // emote
    pub emote_time: f32,
    pub emote_max_time: f32,
    pub is_emoting: bool,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct BasicEntityCollider {
    pub width: f32,
    pub height: f32,
    pub collided: bool,
}

#[derive(Component, Debug, Clone, Copy, Reflect, Resource, Default)]
pub enum DirectionIntent {
    Left,
    Right,
    Up,
    #[default]
    Down,
}

#[derive(Component, Debug, Reflect, Resource, Default, PartialEq)]
pub enum GenericActions {
    #[default]
    Idle,
    Walk,
    Attack,
    IntoRest,
    OutOfRest,
    Resting,
    Sleeping,
    Interacting,
    Eating,
    Emote,
}

#[derive(Component, Debug)]
pub struct InteractionTimers {
    pub prox_chat_cooldown_timer: Timer,
    pub can_prox_chat: bool,

    pub prox_chat_timer: Timer,
    pub prox_chat_active: bool,
}

#[derive(Component, Debug)]
pub struct ChattingIconButton {
    pub chatting_icon_despawn_timer: Timer,
}

#[derive(Component, Debug)]
pub struct EmoteIcon {
    pub emote_icon_despawn_timer: Timer,
}

#[derive(Component, Debug, Clone)]
pub struct IconAnimation {
    pub current_frame: usize,
    pub frame_count: usize,
}
