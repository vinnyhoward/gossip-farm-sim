// packages
use bevy::sprite::collide_aabb::collide;
use csv::Reader;
use rand::Rng;
use serde_json::json;
use std::collections::hash_map::Entry;
use std::fs::File;

use crate::prelude::*;

// components
use crate::gameplay::components::{
    Animation, AnimationTimers, Attack, BasicEntityCollider, ChattingIconButton, Clickable,
    DirectionIntent, EmoteIcon, EntityActionState, GenericActions, IconAnimation,
    InteractionTimers, Movement, Roaming,
};
use crate::gameplay::npc::components::NpcEntity;
use crate::gameplay::player::components::{
    ActiveEtherPet, EmoteParentEntity, EntityPairConversationState, EtherPet,
    PairEntityConversationData, PlayerUniqueId, Quadrants,
};
use crate::gameplay::spritesheet::components::{
    ChattingIconSheet, EmotionIconSheet, PlayerSpriteSheet,
};
use crate::gameplay::state::components::{CameraState, GameState};
use crate::gameplay::tile::components::BasicTileCollider;
use crate::http_request::components::{
    ConversationHistoryResource, HttpMethod, HttpRequest, PendingConversationHttpRequests,
};

// events
use crate::gameplay::events::ConversationActionEvent;
use crate::gameplay::player::events::EmoteEvent;
// use crate::http_request::events::ConversationHistoryEvent;

// helpers
use crate::data::player_data::get_player_data;
use crate::data::spawn_data::get_player_spawn_data;

// systems
use crate::gameplay::spritesheet::systems::{
    spawn_chatting_sprite, spawn_emote_sprite, spawn_player_spritesheet_sprite,
};
use crate::gameplay::tile::systems::basic_wall_collision_check;

fn get_emotion_index(emotion: &str) -> usize {
    if emotion == "Happiness" {
        0
    } else if emotion == "Excitement" {
        1
    } else if emotion == "Sadness" {
        2
    } else if emotion == "Fear" {
        3
    } else if emotion == "Disgust" {
        4
    } else {
        5
    }
}

// movement
pub fn player_confinement_system(mut player_query: Query<&mut Transform, With<EtherPet>>) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let half_player_size: f32 = PLAYER_SIZE / 2.0;
        let half_map_width = SCREEN_WIDTH / 2.0;
        let half_map_height = SCREEN_HEIGHT / 2.0;

        let x_min: f32 = -half_map_width + half_player_size;
        let x_max: f32 = half_map_width - half_player_size;
        let y_min: f32 = -half_map_height + half_player_size;
        let y_max: f32 = half_map_height - half_player_size;

        let mut translation: Vec3 = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn player_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Movement, &mut Transform), With<EtherPet>>,
) {
    for (player_movement, mut transform) in query.iter_mut() {
        let direction = player_movement.direction;

        transform.translation.x += direction.x * player_movement.speed * time.delta_seconds();
        transform.translation.y += direction.y * player_movement.speed * time.delta_seconds();
    }
}

// animation
pub fn player_sprite_animation(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut player_query: Query<
        (
            &mut TextureAtlasSprite,
            &mut Animation,
            &mut EntityActionState,
            &mut AnimationTimers,
        ),
        With<EtherPet>,
    >,
) {
    for (mut spritesheet, mut player_anim, mut player_action_state, mut player_timer) in
        player_query.iter_mut()
    {
        let base_index = match player_anim.last_direction {
            DirectionIntent::Down => match player_action_state.action {
                GenericActions::Idle => 0,
                GenericActions::Walk => 32,
                GenericActions::Attack => 96,
                _ => 0,
            },
            DirectionIntent::Up => match player_action_state.action {
                GenericActions::Idle => 8,
                GenericActions::Walk => 40,
                GenericActions::Attack => 104,
                _ => 0,
            },
            DirectionIntent::Left => match player_action_state.action {
                GenericActions::Idle => 16,
                GenericActions::Walk => 56,
                GenericActions::Attack => 112,
                _ => 0,
            },
            DirectionIntent::Right => match player_action_state.action {
                GenericActions::Idle => 24,
                GenericActions::Walk => 48,
                GenericActions::Attack => 120,
                _ => 0,
            },
        };

        match player_action_state.action {
            GenericActions::Idle => {
                if player_action_state.action != GenericActions::Attack {
                    player_timer.idle_timer.tick(time.delta());
                    if player_timer.idle_timer.just_finished() {
                        player_anim.current_frame =
                            (player_anim.current_frame + 1) % player_anim.frame_count;
                    }
                }
            }
            GenericActions::Walk => {
                if player_action_state.action != GenericActions::Attack {
                    player_timer.walk_timer.tick(time.delta());
                    if player_timer.walk_timer.just_finished() {
                        player_anim.current_frame =
                            (player_anim.current_frame + 1) % player_anim.frame_count;
                    }
                }
            }
            GenericActions::Attack => {
                player_timer.attack_timer.tick(time.delta());
                if player_timer.attack_timer.finished() {
                    player_anim.current_frame =
                        (player_anim.current_frame + 1) % player_anim.frame_count;

                    if player_anim.current_frame == 0 {
                        let attack_sound_effect =
                            asset_server.load("audio/sound_effects/normal_slash.ogg");
                        audio.play(attack_sound_effect);
                        player_timer.attack_timer.reset();
                        player_action_state.action = GenericActions::Idle;
                    }
                }
            }
            GenericActions::IntoRest => {}
            GenericActions::OutOfRest => {}
            GenericActions::Resting => {}
            GenericActions::Sleeping => {}
            GenericActions::Interacting => {}
            GenericActions::Eating => {}
            GenericActions::Emote => {}
        }
        spritesheet.index = base_index + player_anim.current_frame;
    }
}

// player input
pub fn player_input(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut active_player_query: Query<
        (
            Entity,
            &mut Movement,
            &mut Animation,
            &mut EntityActionState,
            &mut Transform,
        ),
        (With<ActiveEtherPet>, Without<BasicTileCollider>),
    >,
    wall_query: Query<&Transform, (With<BasicTileCollider>, Without<EtherPet>)>,
    inactive_player_query: Query<(
        &mut Transform,
        (
            With<BasicEntityCollider>,
            With<NpcEntity>,
            Without<ActiveEtherPet>,
            Without<BasicTileCollider>,
        ),
    )>,
    mut next_camera_state: ResMut<NextState<CameraState>>,
) {
    for (player_entity, player_movement, mut player_anim, mut player_action_state, mut transform) in
        active_player_query.iter_mut()
    {
        if keyboard_input.pressed(KeyCode::Escape) {
            commands.entity(player_entity).remove::<ActiveEtherPet>();
            commands.entity(player_entity).insert(NpcEntity);
            next_camera_state.set(CameraState::ManualCameraControl);
        }

        let mut action = GenericActions::Idle;
        let mut x_delta = 0.0;

        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            x_delta -= player_movement.speed * TILE_SIZE * time.delta_seconds();
            player_anim.last_direction = DirectionIntent::Left;
            action = GenericActions::Walk;
        }

        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            x_delta += player_movement.speed * TILE_SIZE * time.delta_seconds();
            player_anim.last_direction = DirectionIntent::Right;
            action = GenericActions::Walk;
        }

        let mut y_delta = 0.0;
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            y_delta += player_movement.speed * TILE_SIZE * time.delta_seconds();
            player_anim.last_direction = DirectionIntent::Up;
            action = GenericActions::Walk;
        }

        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            y_delta -= player_movement.speed * TILE_SIZE * time.delta_seconds();
            player_anim.last_direction = DirectionIntent::Down;
            action = GenericActions::Walk;
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            action = GenericActions::Attack;
        }

        if player_action_state.action != GenericActions::Attack {
            player_action_state.action = action;
        }

        let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
        if basic_wall_collision_check(target, &wall_query)
            && basic_player_entity_collision_check(target, &inactive_player_query)
        {
            transform.translation = target;
        }

        let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
        if basic_wall_collision_check(target, &wall_query)
            && basic_player_entity_collision_check(target, &inactive_player_query)
        {
            transform.translation = target;
        }
    }
}

// spawner
pub fn player_spawner(mut commands: Commands, player_spritesheet: Res<PlayerSpriteSheet>) {
    let spawn_data = get_player_spawn_data();
    let mut player_amount = 0;
    let all_player_data = get_player_data();

    for spawn_data in spawn_data {
        let spawn_points_file = match File::open(spawn_data.csv_file_path) {
            Ok(file) => file,
            Err(error) => panic!("Failed to open spawn points file: {:?}", error),
        };
        let mut reader = Reader::from_reader(spawn_points_file);
        let mut records: Vec<_> = reader.records().collect();
        let map_height = records.len();
        let map_width = records[0].as_mut().unwrap().len();

        let half_map_width = (map_width as f32 * TILE_SIZE) / 2.0;
        let half_map_height = (map_height as f32 * TILE_SIZE) / 2.0;

        for (y, record_result) in records.iter().enumerate() {
            let record: &csv::StringRecord = record_result.as_ref().unwrap();
            for (x, cell) in record.iter().enumerate() {
                let tile_index = cell.parse::<i32>().unwrap();
                let tile_translation = Vec3::new(
                    x as f32 * TILE_SIZE - half_map_width,
                    -(y as f32) * TILE_SIZE + half_map_height,
                    spawn_data.z_index,
                );

                if tile_index != -1 && player_amount < all_player_data.len() {
                    let player_data = &all_player_data[player_amount];
                    let player_entity = spawn_player_spritesheet_sprite(
                        &mut commands,
                        &player_spritesheet,
                        0,
                        tile_translation,
                    );

                    commands
                        .entity(player_entity)
                        .insert(Name::new(player_data.name.clone()))
                        .insert(EtherPet)
                        .insert(Clickable)
                        .insert(PlayerUniqueId {
                            uuid: player_data.player_id.clone(),
                        })
                        .insert(BasicEntityCollider {
                            width: 16.0,
                            height: 16.0,
                            collided: false,
                        })
                        .insert(AnimationTimers {
                            idle_timer: Timer::from_seconds(0.25, TimerMode::Repeating),
                            walk_timer: Timer::from_seconds(0.075, TimerMode::Repeating),
                            attack_timer: Timer::from_seconds(0.1, TimerMode::Once),
                            eat_timer: Timer::from_seconds(0.075, TimerMode::Repeating),
                            emote_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                        })
                        .insert(EntityActionState {
                            action: GenericActions::Idle,
                        })
                        .insert(Animation {
                            last_direction: DirectionIntent::Down,
                            current_frame: 0,
                            frame_count: 8,
                        })
                        .insert(Attack {
                            attack_active: false,
                            attack_dmg: player_data.attack_dmg,
                        })
                        .insert(Movement {
                            direction: Vec2::ZERO,
                            speed: player_data.speed,
                        })
                        .insert(Roaming {
                            // roam
                            roam_direction: Vec3::ZERO,
                            roam_time: 0.0,
                            roam_max_time: 5.0,
                            roam_speed: player_data.speed,
                            // idle
                            is_idle: false,
                            idle_time: 0.0,
                            idle_max_time: 2.0,
                            // eat
                            is_eating: false,
                            eating_time: 0.0,
                            eating_max_time: 3.0,
                            // emote
                            is_emoting: false,
                            emote_time: 0.0,
                            emote_max_time: 3.0,
                        })
                        .insert(InteractionTimers {
                            // when character is chatting with another character
                            prox_chat_timer: Timer::from_seconds(
                                ENTITY_CONVO_DURATION,
                                TimerMode::Once,
                            ),
                            prox_chat_active: false,
                            // when cooldown timer is active characters cannot chat
                            prox_chat_cooldown_timer: Timer::from_seconds(15.0, TimerMode::Once),
                            can_prox_chat: true,
                        })
                        .insert(NpcEntity);

                    player_amount += 1;
                }

                if player_amount == all_player_data.len() {
                    break;
                }
            }
        }
    }
}

pub fn basic_player_entity_collision_check(
    target_player_pos: Vec3,
    inactive_player_query: &Query<(
        &mut Transform,
        (
            With<BasicEntityCollider>,
            With<NpcEntity>,
            Without<ActiveEtherPet>,
            Without<BasicTileCollider>,
        ),
    )>,
) -> bool {
    for in_active_player in inactive_player_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE * 0.01),
            in_active_player.0.translation,
            Vec2::splat(TILE_SIZE),
        );
        if collision.is_some() {
            return false;
        }
    }
    return true;
}

// other
pub fn player_activation_system(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut windows: Query<&mut Window>,
    mut camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut player_query: Query<(Entity, &Transform), (With<EtherPet>, With<Clickable>)>,
) {
    let (camera, camera_transform) = camera_query.single_mut();
    let window = windows.single_mut();

    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(world_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin)
        {
            for (player_entity, transform) in player_query.iter_mut() {
                let player_pos = transform.translation;
                let player_size = Vec2::new(16.0, 16.0);
                let collision = collide(world_pos, Vec2::ZERO, player_pos, player_size);

                if collision.is_some() {
                    commands.entity(player_entity).insert(ActiveEtherPet);
                    commands.entity(player_entity).remove::<NpcEntity>();
                } else {
                    commands.entity(player_entity).remove::<ActiveEtherPet>();
                    commands.entity(player_entity).insert(NpcEntity);
                }
            }
        }
    }
}

fn find_quadrant(entity_a: Vec3, entity_b: Vec3) -> Quadrants {
    if entity_b.y < entity_a.y && entity_b.x > entity_a.x {
        return Quadrants::BottomRight;
    } else if entity_b.y < entity_a.y && entity_b.x < entity_a.x {
        return Quadrants::BottomLeft;
    } else if entity_b.y > entity_a.y && entity_b.x > entity_a.x {
        return Quadrants::TopRight;
    } else if entity_b.y > entity_a.y && entity_b.x < entity_a.x {
        return Quadrants::TopLeft;
    } else {
        return Quadrants::None;
    }
}

// TODO: Can probably remove in the near future and explicitly do it
pub fn update_app_state_system(
    mut next_camera_state: ResMut<NextState<CameraState>>,
    camera_state: Res<State<CameraState>>,
    game_state: Res<State<GameState>>,
    player_query: Query<Entity, (With<EtherPet>, With<Clickable>, With<ActiveEtherPet>)>,
) {
    let active_player_exists = player_query.iter().count() > 0;

    if active_player_exists
        && camera_state.0 != CameraState::FollowPlayer
        && game_state.0 == GameState::Playing
    {
        next_camera_state.set(CameraState::FollowPlayer);
    } else if !active_player_exists
        && camera_state.0 != CameraState::ManualCameraControl
        && game_state.0 == GameState::Playing
    {
        next_camera_state.set(CameraState::ManualCameraControl);
    }
}

// TODO: This system function needs to be refactored with the new proximity system
// that isn't a double for loop
pub fn inactive_player_proximity_detection(
    mut entity_pair_convo_state: ResMut<EntityPairConversationState>,
    inactive_player_query: Query<
        (
            Entity,
            &Transform,
            &InteractionTimers,
            &Animation,
            &PlayerUniqueId,
        ),
        (With<EtherPet>, Without<ActiveEtherPet>),
    >,
    mut http_request_event_writer: EventWriter<ConversationActionEvent>,
) {
    let inactive_players: Vec<(
        Entity,
        Vec3,
        &InteractionTimers,
        &Animation,
        &PlayerUniqueId,
    )> = inactive_player_query
        .iter()
        .filter(|(_, _, interaction_timers, _, _)| !interaction_timers.prox_chat_active)
        .map(
            |(entity, transform, interaction_timers, entity_animation, unique_id)| {
                (
                    entity,
                    transform.translation,
                    interaction_timers,
                    entity_animation,
                    unique_id,
                )
            },
        )
        .collect();

    for i in 0..inactive_players.len() {
        for j in i + 1..inactive_players.len() {
            let (entity_a, pos_a, interaction_timer_a, _, entity_a_id) = inactive_players[i];
            let (entity_b, pos_b, interaction_timer_b, _, entity_b_id) = inactive_players[j];

            if entity_a != entity_b
                && !interaction_timer_a.prox_chat_active
                && !interaction_timer_b.prox_chat_active
            {
                // // Skip the pair if either entity is already in a conversation.
                if *entity_pair_convo_state
                    .entities_in_conversation
                    .get(&entity_a)
                    .unwrap_or(&false)
                    || *entity_pair_convo_state
                        .entities_in_conversation
                        .get(&entity_b)
                        .unwrap_or(&false)
                {
                    continue;
                }

                let distance_from_each_other = pos_a.distance(pos_b);
                let quadrant = find_quadrant(pos_a, pos_b);
                if distance_from_each_other < ENTITY_CONVO_DISTANCE
                    && distance_from_each_other > 10.
                    && interaction_timer_a.can_prox_chat
                    && interaction_timer_b.can_prox_chat
                {
                    match entity_pair_convo_state
                        .pair_entities_in_conversation
                        .entry((entity_a, entity_b))
                    {
                        Entry::Occupied(_) => {}
                        Entry::Vacant(entry) => {
                            entry.insert(PairEntityConversationData {
                                entity_a,
                                entity_b,
                                entity_a_pos: pos_a,
                                entity_b_pos: pos_b,
                                entity_a_id: entity_a_id.uuid.clone(),
                                entity_b_id: entity_b_id.uuid.clone(),
                                quadrant,
                                chatting_icon_spawned: false,
                                conversation_http_request_sent: false,
                            });

                            // Also mark both entities as in use.
                            entity_pair_convo_state
                                .entities_in_conversation
                                .insert(entity_a, true);
                            entity_pair_convo_state
                                .entities_in_conversation
                                .insert(entity_b, true);
                            // TODO: Add logic to stop event from firing if player is going to hit a collider
                            http_request_event_writer
                                .send(ConversationActionEvent { entity_a, entity_b });
                        }
                    }
                } else {
                    entity_pair_convo_state
                        .pair_entities_in_conversation
                        .remove(&(entity_a, entity_b));

                    // Also mark both entities as not in use.
                    entity_pair_convo_state
                        .entities_in_conversation
                        .insert(entity_a, false);
                    entity_pair_convo_state
                        .entities_in_conversation
                        .insert(entity_b, false);
                }
            }
        }
    }
}

pub fn pair_entity_convergence_system(
    mut entity_pair_convo_state: ResMut<EntityPairConversationState>,
    mut entities_query: Query<
        (
            &mut Transform,
            &mut EntityActionState,
            &mut Animation,
            &Movement,
            &PlayerUniqueId,
        ),
        (With<EtherPet>, Without<ActiveEtherPet>),
    >,
    wall_query: Query<&Transform, (With<BasicTileCollider>, Without<EtherPet>)>,
) {
    let mut entities_to_update = Vec::new();
    let mut entities_to_remove = Vec::new();

    // First loop: gather all data
    for ((entity_a, entity_b), pair_convo_data) in
        entity_pair_convo_state.pair_entities_in_conversation.iter()
    {
        if let Ok((transform_a, mut entity_a_action_state, mut animation_a, _, entity_a_uuid)) =
            entities_query.get_mut(*entity_a)
        {
            let entity_a_uuid = entity_a_uuid.uuid.clone();
            entity_a_action_state.action = GenericActions::Idle;
            match pair_convo_data.quadrant {
                Quadrants::TopLeft => {
                    animation_a.last_direction = DirectionIntent::Up;
                }
                Quadrants::TopRight => {
                    animation_a.last_direction = DirectionIntent::Right;
                }
                Quadrants::BottomLeft => {
                    animation_a.last_direction = DirectionIntent::Down;
                }
                Quadrants::BottomRight => {
                    animation_a.last_direction = DirectionIntent::Left;
                }
                Quadrants::None => {}
            }

            entities_to_update.push((
                *entity_a,
                *entity_b,
                transform_a.translation,
                pair_convo_data,
                entity_a_uuid,
            ));
        }
    }

    // Second loop: perform updates based on gathered data
    for (_, entity_b, entity_a_transform, pair_convo_data, entity_a_uuid) in
        entities_to_update.iter()
    {
        if let Ok((
            mut transform_b,
            mut entity_b_action_state,
            mut animation_b,
            movement_b,
            entity_b_uuid,
        )) = entities_query.get_mut(*entity_b)
        {
            let _entity_b_uuid = entity_b_uuid.uuid.clone();
            let _entity_a_uuid = entity_a_uuid.clone();

            match pair_convo_data.quadrant {
                Quadrants::TopLeft => {
                    animation_b.last_direction = DirectionIntent::Down;
                    if transform_b.translation.x.round() == entity_a_transform.x.round() {
                        entity_b_action_state.action = GenericActions::Idle;
                    } else {
                        entity_b_action_state.action = GenericActions::Walk;
                        let x_value = entity_a_transform.x;
                        let y_value = entity_a_transform.y + ENTITY_CONVO_DISTANCE;
                        let target = Vec3::new(x_value, y_value, 0.0);
                        if basic_wall_collision_check(target, &wall_query) {
                            let direction_to_target = target - transform_b.translation;
                            let delta = direction_to_target.normalize() * movement_b.speed;

                            transform_b.translation.x += delta.x;
                            transform_b.translation.y += delta.y;
                        } else {
                            entity_b_action_state.action = GenericActions::Idle;
                            entities_to_remove.push(*entity_b);
                        }
                    }
                }
                Quadrants::TopRight => {
                    animation_b.last_direction = DirectionIntent::Left;
                    if transform_b.translation.y.round() == entity_a_transform.y.round() {
                        entity_b_action_state.action = GenericActions::Idle;
                    } else {
                        entity_b_action_state.action = GenericActions::Walk;
                        let y_value = entity_a_transform.y;
                        let x_value = entity_a_transform.x + ENTITY_CONVO_DISTANCE;

                        let target = Vec3::new(x_value, y_value, 0.0);
                        if basic_wall_collision_check(target, &wall_query) {
                            let direction_to_target = target - transform_b.translation;
                            let delta = direction_to_target.normalize() * movement_b.speed;

                            transform_b.translation.x += delta.x;
                            transform_b.translation.y += delta.y;
                        } else {
                            entity_b_action_state.action = GenericActions::Idle;
                            entities_to_remove.push(*entity_b);
                        }
                    }
                }
                Quadrants::BottomLeft => {
                    animation_b.last_direction = DirectionIntent::Up;
                    if transform_b.translation.x.round() == entity_a_transform.x.round() {
                        entity_b_action_state.action = GenericActions::Idle;
                    } else {
                        entity_b_action_state.action = GenericActions::Walk;
                        let x_value = entity_a_transform.x;
                        let y_value = entity_a_transform.y - ENTITY_CONVO_DISTANCE;

                        let target = Vec3::new(x_value, y_value, 0.0);
                        if basic_wall_collision_check(target, &wall_query) {
                            let direction_to_target = target - transform_b.translation;
                            let delta = direction_to_target.normalize() * movement_b.speed;

                            transform_b.translation.x += delta.x;
                            transform_b.translation.y += delta.y;
                        } else {
                            entity_b_action_state.action = GenericActions::Idle;
                            entities_to_remove.push(*entity_b);
                        }
                    }
                }
                Quadrants::BottomRight => {
                    animation_b.last_direction = DirectionIntent::Right;
                    if transform_b.translation.y.round() == entity_a_transform.y.round() {
                        entity_b_action_state.action = GenericActions::Idle;
                    } else {
                        entity_b_action_state.action = GenericActions::Walk;
                        let y_value = entity_a_transform.y;
                        let x_value = entity_a_transform.x - 12.;

                        let target = Vec3::new(x_value, y_value, 0.0);
                        if basic_wall_collision_check(target, &wall_query) {
                            let direction_to_target = target - transform_b.translation;
                            let delta = direction_to_target.normalize() * movement_b.speed;

                            transform_b.translation.x += delta.x;
                            transform_b.translation.y += delta.y;
                        } else {
                            entity_b_action_state.action = GenericActions::Idle;
                            entities_to_remove.push(*entity_b);
                        }
                    }
                }
                Quadrants::None => {}
            }
        }
    }

    // Finally, remove entities as needed
    for entity in entities_to_remove {
        remove_entity_from_conversation(entity, &mut entity_pair_convo_state);
    }
}

pub fn inactive_player_proximity_detection_event(
    mut action_event_reader: EventReader<ConversationActionEvent>,
    mut entities_query: Query<&mut InteractionTimers, (With<EtherPet>, Without<ActiveEtherPet>)>,
) {
    for event in action_event_reader.iter() {
        println!("Event: {:?}", event);

        if let Ok(mut inactive_player_a) = entities_query.get_mut(event.entity_a) {
            if inactive_player_a.can_prox_chat {
                inactive_player_a.prox_chat_active = true;
            }
        }

        if let Ok(mut inactive_player_b) = entities_query.get_mut(event.entity_b) {
            if inactive_player_b.can_prox_chat {
                inactive_player_b.prox_chat_active = true;
            }
        }
    }
}

pub fn tick_player_interaction_timers(
    time: Res<Time>,
    mut inactive_player_query: Query<
        (Entity, &mut InteractionTimers),
        (Without<ActiveEtherPet>, With<EtherPet>),
    >,
    mut entity_pair_convo_state: ResMut<EntityPairConversationState>,
    mut conversation_response_resource: ResMut<ConversationHistoryResource>,
    mut emote_event_write: EventWriter<EmoteEvent>,
) {
    let delta_seconds = time.delta();
    for (entity, mut interaction_timers) in inactive_player_query.iter_mut() {
        if !interaction_timers.can_prox_chat {
            interaction_timers
                .prox_chat_cooldown_timer
                .tick(delta_seconds);
        }

        if interaction_timers.prox_chat_cooldown_timer.finished() {
            interaction_timers.can_prox_chat = true;
        }

        if interaction_timers.can_prox_chat && interaction_timers.prox_chat_active {
            interaction_timers.prox_chat_timer.tick(delta_seconds);
        }

        if interaction_timers.prox_chat_timer.finished() {
            interaction_timers.prox_chat_active = false;
            interaction_timers.can_prox_chat = false;

            interaction_timers.prox_chat_timer.reset();
            interaction_timers.prox_chat_cooldown_timer.reset();

            // TODO:  Query by entity id then fire this off other wise it can fire others prematurely
            while let Some(response) = conversation_response_resource.entities_to_emote.pop_front()
            {
                emote_event_write.send(EmoteEvent {
                    entity_uuid: response.entity_uuid.clone(),
                    emote: response.emote.clone(),
                });
            }

            remove_entity_from_conversation(entity, &mut entity_pair_convo_state);
        }
    }
}

pub fn inactive_player_roaming_system(
    time: Res<Time>,
    wall_query: Query<&Transform, (With<BasicTileCollider>, Without<EtherPet>)>,
    mut npc_query: Query<
        (
            &mut Animation,
            &mut Roaming,
            &mut EntityActionState,
            &mut Transform,
            &mut BasicEntityCollider,
            &mut InteractionTimers,
        ),
        (
            With<NpcEntity>,
            With<BasicEntityCollider>,
            With<EtherPet>,
            Without<ActiveEtherPet>,
            Without<BasicTileCollider>,
        ),
    >,
) {
    for (
        mut entity_anim,
        mut entity_roaming_data,
        mut entity_action_state,
        mut transform,
        mut collision_state,
        interaction_timers,
    ) in npc_query.iter_mut()
    {
        if interaction_timers.prox_chat_active == false {
            // Emote
            if entity_roaming_data.is_emoting {
                entity_action_state.action = GenericActions::Emote;
                entity_roaming_data.emote_time += time.delta_seconds();
                if entity_roaming_data.emote_time > entity_roaming_data.emote_max_time {
                    entity_roaming_data.is_emoting = false;
                    entity_roaming_data.emote_time = 0.0;
                }
                continue;
            }

            // Eating
            if entity_roaming_data.is_eating {
                entity_action_state.action = GenericActions::Eating;
                entity_roaming_data.eating_time += time.delta_seconds();
                if entity_roaming_data.eating_time > entity_roaming_data.eating_max_time {
                    entity_roaming_data.is_eating = false;
                    entity_roaming_data.eating_time = 0.0;
                }
                continue;
            }

            // Roaming
            if entity_roaming_data.is_idle {
                entity_action_state.action = GenericActions::Idle;
                entity_roaming_data.idle_time += time.delta_seconds();
                if entity_roaming_data.idle_time > entity_roaming_data.idle_max_time {
                    entity_roaming_data.is_idle = false;
                    entity_roaming_data.idle_time = 0.0;
                }
                continue;
            }

            entity_roaming_data.roam_time += time.delta_seconds();

            let target = transform.translation
                + entity_roaming_data.roam_direction
                    * entity_roaming_data.roam_speed
                    * TILE_SIZE
                    * time.delta_seconds();

            if entity_roaming_data.roam_time > entity_roaming_data.roam_max_time
                || entity_roaming_data.eating_time > entity_roaming_data.eating_max_time
                || entity_roaming_data.emote_time > entity_roaming_data.emote_max_time
                || entity_roaming_data.roam_direction == Vec3::ZERO
                || !basic_wall_collision_check(target, &wall_query)
                || collision_state.collided
            {
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.3) {
                    continue;
                } else {
                    entity_roaming_data.roam_direction =
                        Vec3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), 0.0)
                            .normalize();
                    entity_roaming_data.roam_time = 0.0;
                    entity_anim.last_direction = match (
                        entity_roaming_data.roam_direction.x.signum(),
                        entity_roaming_data.roam_direction.y.signum(),
                    ) {
                        (x, y) if x > 0.0 && y.abs() <= x => DirectionIntent::Right,
                        (x, y) if x < 0.0 && y.abs() <= -x => DirectionIntent::Left,
                        (_, y) if y > 0.0 => DirectionIntent::Up,
                        _ => DirectionIntent::Down,
                    };
                    entity_action_state.action = GenericActions::Walk;
                }
            }

            // Roam
            if basic_wall_collision_check(target, &wall_query) && !collision_state.collided {
                transform.translation = target;
                entity_action_state.action = GenericActions::Walk;
            } else {
                entity_roaming_data.is_idle = true;
                entity_action_state.action = GenericActions::Idle;
            }

            collision_state.collided = false;
        }
    }
}

pub fn chatting_icon_tick(
    mut commands: Commands,
    time: Res<Time>,
    mut chatting_icons_query: Query<(Entity, &mut TextureAtlasSprite, &mut ChattingIconButton)>,
) {
    let delta_seconds = time.delta();
    for (chatting_icon_entity, mut spritesheet, mut chatting_icon_button) in
        chatting_icons_query.iter_mut()
    {
        chatting_icon_button
            .chatting_icon_despawn_timer
            .tick(delta_seconds);

        let elapsed = chatting_icon_button
            .chatting_icon_despawn_timer
            .elapsed()
            .as_secs_f32();
        let num_frames = 4;
        let total_duration = ENTITY_CONVO_DURATION * 0.8;
        let frame_duration = total_duration / (num_frames - 1) as f32 / 4.5;

        let base_index = if elapsed < frame_duration {
            (elapsed / frame_duration * (num_frames - 1) as f32).round() as usize
        } else if elapsed >= total_duration - frame_duration {
            num_frames
                + ((elapsed - (total_duration - frame_duration)) / frame_duration).round() as usize
        } else {
            let chatting_frame_index = ((elapsed - frame_duration) / frame_duration
                * (num_frames - 1) as f32)
                .round() as usize;
            8 + (chatting_frame_index % num_frames)
        };

        spritesheet.index = base_index;

        if chatting_icon_button.chatting_icon_despawn_timer.finished() {
            return commands.entity(chatting_icon_entity).despawn();
        }
    }
}

pub fn chatting_icon_spawner(
    mut commands: Commands,
    mut entity_pair_convo_state: ResMut<EntityPairConversationState>,
    chatting_icon_sheet: Res<ChattingIconSheet>,
) {
    for (_, pair_convo_data) in entity_pair_convo_state
        .pair_entities_in_conversation
        .iter_mut()
    {
        if !pair_convo_data.chatting_icon_spawned {
            let icon_translation: Vec3 = match pair_convo_data.quadrant {
                Quadrants::TopLeft => pair_convo_data.entity_a_pos + Vec3::new(0.0, 20., 0.0),
                Quadrants::TopRight => {
                    pair_convo_data.entity_a_pos + Vec3::new(ENTITY_CONVO_DISTANCE / 2., 15.0, 0.0)
                }
                Quadrants::BottomLeft => pair_convo_data.entity_a_pos + Vec3::new(0.0, 15.0, 0.0),
                Quadrants::BottomRight => {
                    pair_convo_data.entity_a_pos
                        + Vec3::new(-(ENTITY_CONVO_DISTANCE / 2.), 15.0, 0.0)
                }
                Quadrants::None => Vec3::ZERO,
            };
            let chatting_icon_entity: Entity =
                spawn_chatting_sprite(&mut commands, &chatting_icon_sheet, 0, icon_translation);

            commands
                .entity(chatting_icon_entity)
                .insert(ChattingIconButton {
                    chatting_icon_despawn_timer: Timer::from_seconds(
                        ENTITY_CONVO_DURATION * 0.9,
                        TimerMode::Once,
                    ),
                })
                .insert(IconAnimation {
                    current_frame: 0,
                    frame_count: 4,
                });
            pair_convo_data.chatting_icon_spawned = true;
        }
    }
}

pub fn chatting_icon_on_hover(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut windows: Query<&mut Window>,
    mut camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mut chatting_icons_query: Query<(Entity, &Transform), With<ChattingIconButton>>,
) {
    let (camera, camera_transform) = camera_query.single_mut();
    let window: Mut<'_, Window> = windows.single_mut();

    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(world_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin)
        {
            for (_, transform) in chatting_icons_query.iter_mut() {
                let icon_pos = transform.translation;
                let icon_size = Vec2::new(16.0, 16.0);
                let collision = collide(world_pos, Vec2::ZERO, icon_pos, icon_size);

                if collision.is_some() {
                    println!("Chatting icon clicked!");
                }
            }
        }
    }
}

fn spawn_emote(
    mut commands: &mut Commands,
    entities_query: &Query<(Entity, &mut Transform, &PlayerUniqueId), With<EtherPet>>,
    entity_id: String,
    emotion_index: usize,
    emotion_icon_sheet: &Res<EmotionIconSheet>,
) {
    let pair_entities: Vec<(Entity, Transform, String)> = entities_query
        .iter()
        .filter(|(_, _, entity_unique_id)| entity_unique_id.uuid == entity_id.clone())
        .map(|(entity, transform, entity_unique_id)| {
            (entity, *transform, entity_unique_id.uuid.clone())
        })
        .collect();

    for (entity, transform, _entity_unique_id) in pair_entities {
        let emote_translation = transform.translation + Vec3::new(0.0, 17.5, 1000.0);
        let emote_entity: Entity = spawn_emote_sprite(
            &mut commands,
            &emotion_icon_sheet,
            emotion_index,
            emote_translation,
        );

        commands
            .entity(emote_entity)
            .insert(EmoteIcon {
                emote_icon_despawn_timer: Timer::from_seconds(
                    ENTITY_CONVO_DURATION * 0.5,
                    TimerMode::Once,
                ),
            })
            .insert(IconAnimation {
                current_frame: 0,
                frame_count: 4,
            })
            .insert(EmoteParentEntity { entity });
    }
}

pub fn emote_event(
    mut commands: Commands,
    mut emote_event_reader: EventReader<EmoteEvent>,
    entities_query: Query<(Entity, &mut Transform, &PlayerUniqueId), With<EtherPet>>,
    emotion_icon_sheet: Res<EmotionIconSheet>,
) {
    for event in emote_event_reader.iter() {
        let entity_uuid = &event.entity_uuid;
        let emotion_index = get_emotion_index(&event.emote);

        spawn_emote(
            &mut commands,
            &entities_query,
            entity_uuid.clone(),
            emotion_index,
            &emotion_icon_sheet,
        );
    }
}

pub fn emote_movement_system(
    mut emotes: Query<(&EmoteParentEntity, &mut Transform), With<EmoteIcon>>,
    entities: Query<&Transform, (With<EtherPet>, Without<EmoteIcon>)>,
) {
    for (emote_parent, mut emote_transform) in emotes.iter_mut() {
        if let Ok(entity_transform) = entities.get(emote_parent.entity) {
            emote_transform.translation = entity_transform.translation + Vec3::new(0.0, 17.5, 0.0);
        }
    }
}

pub fn emote_icon_tick(
    mut commands: Commands,
    time: Res<Time>,
    mut emote_icon_query: Query<(Entity, &mut TextureAtlas, &mut EmoteIcon)>,
) {
    let delta_seconds = time.delta();
    for (emote_icon_entity, _, mut emote_icon) in emote_icon_query.iter_mut() {
        emote_icon.emote_icon_despawn_timer.tick(delta_seconds);
        if emote_icon.emote_icon_despawn_timer.finished() {
            return commands.entity(emote_icon_entity).despawn();
        }
    }
}

fn remove_entity_from_conversation(
    entity: Entity,
    entity_pair_convo_state: &mut ResMut<EntityPairConversationState>,
) {
    entity_pair_convo_state
        .pair_entities_in_conversation
        .retain(|&(a, b), _| a != entity && b != entity);

    entity_pair_convo_state
        .entities_in_conversation
        .retain(|&a, _| a != entity);
}

pub fn queue_pair_entity_conversation_request(// mut entity_pair_convo_state: ResMut<EntityPairConversationState>,
    // mut pending_requests: ResMut<PendingConversationHttpRequests>,
) {
    // for (_, pair_convo_data) in entity_pair_convo_state
    //     .pair_entities_in_conversation
    //     .iter_mut()
    // {
    //     if !pair_convo_data.conversation_http_request_sent {
    //         let body = json!({
    //             "entity_a_uuid": pair_convo_data.entity_a_id,
    //             "entity_b_uuid": pair_convo_data.entity_b_id,
    //         });

    //         pending_requests.requests.push(HttpRequest {
    //             url: API_URL.to_string() + "/characters/converse",
    //             method: HttpMethod::POST,
    //             headers: HashMap::new(),
    //             body,
    //         });
    //         pair_convo_data.conversation_http_request_sent = true;
    //     }
    // }
}
