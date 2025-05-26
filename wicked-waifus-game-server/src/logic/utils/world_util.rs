use wicked_waifus_protocol::{
    EEntityType, ERemoveEntityType, EntityAddNotify, EntityConfigType, EntityPb, EntityRemoveInfo,
    EntityRemoveNotify, EntityState, FightRoleInfo, FightRoleInfos, LivingStatus, SceneInformation,
    SceneMode, ScenePlayerInformation, SceneTimeInfo,
};

use wicked_waifus_data::pb_components::ComponentsData;
use wicked_waifus_data::{
    blueprint_config_data, template_config_data, EntityLogic, EntityType, LevelEntityConfigData,
};

use crate::logic::components::{Autonomous, Fsm, Interact, MonsterAi, ParaglidingSkin, SoarWingSkin, StateTag, Tag, WeaponSkin};
use crate::logic::ecs::entity::EntityBuilder;
use crate::logic::ecs::world::World;
use crate::logic::math::Transform;
use crate::logic::player::Player;
use crate::logic::utils::{entity_serializer, tag_utils};
use crate::logic::utils::growth_utils::get_monster_props_by_level;
use crate::logic::{
    components::{
        Attribute, EntityConfig, Equip, FightBuff, Movement, OwnerPlayer, PlayerOwnedEntityMarker,
        Position, RoleSkin, Visibility, VisionSkill,
    },
    ecs::component::ComponentContainer,
};
use crate::query_with;

#[macro_export]
macro_rules! create_player_entity_pb {
    ($role_list:expr, $cur_map_id:expr, $player:expr, $player_id:expr, $position:expr, $explore_tools:expr, $world:expr) => {{
        let current_formation = $player.formation_list.get(&$player.cur_formation_id).unwrap();
        let cur_role_id = current_formation.cur_role;

        let mut pbs = Vec::new();

        for role in $role_list {
            let entity = $world.create_entity(
                role.role_id,
                EEntityType::Player.into(),
                $cur_map_id,
            );
            // Once per character buffs are implemented, add a mut on role_buffs
            let fight_buff_infos = $world.generate_role_permanent_buffs(entity.entity_id as i64);
            let buf_manager = FightBuff {
                fight_buff_infos,
                list_buff_effect_cd: vec![],
            };

            let entity = $world.create_builder(entity)
                .with(ComponentContainer::PlayerOwnedEntityMarker(PlayerOwnedEntityMarker {
                    entity_type: EEntityType::Player,
                }))
                .with(ComponentContainer::EntityConfig(EntityConfig {
                    camp: 0,
                    config_id: role.role_id,
                    config_type: EntityConfigType::Character,
                    entity_type: EEntityType::Player.into(),
                    entity_state: EntityState::Default,
                }))
                .with(ComponentContainer::OwnerPlayer(OwnerPlayer($player_id)))
                .with(ComponentContainer::Position(Position($position)))
                .with(ComponentContainer::Visibility(Visibility{
                    is_visible: role.role_id == cur_role_id,
                    is_actor_visible: true,
                }))
                // TODO: Check if role has hardness or rage_mode
                // TODO: Support AddProp from Equipment(Echo, weapon, buffs??), weapon base state goes to base_prop too.
                .with(ComponentContainer::Attribute(
                     Attribute::from_data(
                         &role.get_base_properties(),
                         None,
                         None,
                     )
                ))
                .with(ComponentContainer::Movement(Movement::default()))
                .with(ComponentContainer::Equip(Equip {
                    weapon_id: role.equip_weapon,
                    weapon_breach_level: 90, // TODO: store this too
                }))
                .with(ComponentContainer::VisionSkill(VisionSkill {
                    skill_id: $explore_tools.active_explore_skill,
                }))
                .with(ComponentContainer::RoleSkin(RoleSkin {
                    skin_id: role.skin_id,
                }))
                .with(ComponentContainer::SoarWingSkin(SoarWingSkin {
                    skin_id: role.fly_skin_id,
                }))
                .with(ComponentContainer::ParaglidingSkin(ParaglidingSkin {
                    skin_id: role.wing_skin_id,
                }))
                .with(ComponentContainer::WeaponSkin(WeaponSkin {
                    skin_id: role.weapon_skin_id,
                }))
                .with(ComponentContainer::FightBuff(buf_manager))
                .build();

            let mut pb = EntityPb {
                id: entity.entity_id as i64,
                ..Default::default()
            };

            $world
                .get_entity_components(entity.entity_id)
                .into_iter()
                .for_each(|comp| comp.set_pb_data(&mut pb));
            pbs.push(pb);
        }

        EntityAddNotify {
            entity_pbs: pbs,
            remove_tag_ids: true,
        }
    }};
}

pub fn add_player_entities(player: &Player) {
    let mut world_ref = player.world.borrow_mut();
    let world = world_ref.get_mut_world_entity();

    let current_formation = player.formation_list.get(&player.cur_formation_id).unwrap();

    let role_vec = current_formation
        .role_ids
        .iter()
        .map(|role_id| player.role_list.get(&role_id).unwrap())
        .collect::<Vec<_>>();
    let cur_role_id = current_formation.cur_role;

    if world.active_entity_empty() {
        for role in role_vec {
            let entity = world.create_entity(
                role.role_id,
                EEntityType::Player.into(),
                player.basic_info.cur_map_id,
            );
            // Once per character buffs are implemented, add a mut on role_buffs
            let fight_buff_infos = world.generate_role_permanent_buffs(entity.entity_id as i64);
            let buf_manager = FightBuff {
                fight_buff_infos,
                list_buff_effect_cd: vec![],
            };
            let entity = world
                .create_builder(entity)
                .with(ComponentContainer::PlayerOwnedEntityMarker(
                    PlayerOwnedEntityMarker {
                        entity_type: EEntityType::Player,
                    },
                ))
                .with(ComponentContainer::EntityConfig(EntityConfig {
                    camp: 0,
                    config_id: role.role_id,
                    config_type: EntityConfigType::Character,
                    entity_type: EEntityType::Player.into(),
                    entity_state: EntityState::Default,
                }))
                .with(ComponentContainer::OwnerPlayer(OwnerPlayer(
                    player.basic_info.id,
                )))
                .with(ComponentContainer::Position(Position(
                    player.location.position.clone(),
                )))
                .with(ComponentContainer::Visibility(Visibility {
                    is_visible: role.role_id == cur_role_id,
                    is_actor_visible: true,
                }))
                // TODO: from role
                // TODO: Check if role has hardness or rage_mode
                // TODO: Support AddProp from Equipment(Echo, weapon, buffs??), weapon base state goes to base_prop too.
                .with(ComponentContainer::Attribute(Attribute::from_data(
                    &role.get_base_properties(),
                    None,
                    None,
                )))
                .with(ComponentContainer::Movement(Movement::default()))
                .with(ComponentContainer::Equip(Equip {
                    weapon_id: role.equip_weapon,
                    weapon_breach_level: 0, // TODO: store this too
                }))
                .with(ComponentContainer::VisionSkill(VisionSkill {
                    skill_id: player.explore_tools.active_explore_skill,
                }))
                .with(ComponentContainer::RoleSkin(RoleSkin {
                    skin_id: role.skin_id,
                }))
                .with(ComponentContainer::SoarWingSkin(SoarWingSkin {
                    skin_id: role.fly_skin_id,
                }))
                .with(ComponentContainer::ParaglidingSkin(ParaglidingSkin {
                    skin_id: role.wing_skin_id,
                }))
                .with(ComponentContainer::WeaponSkin(WeaponSkin {
                    skin_id: role.weapon_skin_id, // TODO: Is this kept on weapon change
                }))
                .with(ComponentContainer::FightBuff(buf_manager))
                .build();

            tracing::debug!(
                "created player entity, id: {}, role_id: {}",
                entity.entity_id,
                role.role_id
            );
        }
    }
}

pub fn build_scene_information(player: &Player) -> SceneInformation {
    SceneInformation {
        scene_id: String::new(),
        instance_id: player.location.instance_id,
        owner_id: player.basic_info.id,
        dynamic_entity_list: Vec::new(),
        blackboard_params: Vec::new(),
        end_time: 0,
        aoi_data: Some(entity_serializer::build_scene_add_on_init_data(player)),
        player_infos: build_player_info_list(&player.world.borrow_mut()),
        mode: SceneMode::Single.into(),
        time_info: Some(SceneTimeInfo {
            owner_time_clock_time_span: 0,
            hour: 8,
            minute: 0,
        }),
        cur_context_id: player.basic_info.id as i64,
        ..Default::default()
    }
}

fn build_player_info_list(world: &World) -> Vec<ScenePlayerInformation> {
    world
        .players()
        .map(|sp| {
            let (cur_role_id, transform, _equip) = query_with!(
                world.get_world_entity(),
                PlayerOwnedEntityMarker,
                OwnerPlayer,
                Visibility,
                EntityConfig,
                Position,
                Equip
            )
            .into_iter()
            .find_map(|(_, _, owner, visibility, conf, pos, equip)| {
                (sp.player_id == owner.0 && visibility.is_visible).then_some((
                    conf.config_id,
                    pos.0.clone(),
                    equip.weapon_id,
                ))
            })
            .unwrap_or_default();

            let active_characters = query_with!(
                world.get_world_entity(),
                PlayerOwnedEntityMarker,
                OwnerPlayer,
                EntityConfig
            )
            .into_iter()
            .filter(|(_, _, owner, _)| owner.0 == sp.player_id);

            ScenePlayerInformation {
                cur_role: cur_role_id,
                group_type: sp.group_type,
                player_id: sp.player_id,
                player_icon: sp.player_icon,
                player_name: sp.player_name.clone(),
                level: sp.level,
                location: Some(transform.get_position_protobuf()),
                rotation: Some(transform.get_rotation_protobuf()),
                fight_role_infos: Vec::from([FightRoleInfos {
                    group_type: sp.group_type,
                    living_status: LivingStatus::Alive.into(),
                    cur_role: cur_role_id,
                    // is_retain: true,
                    fight_role_infos: active_characters
                        .map(|(id, _, _, conf)| FightRoleInfo {
                            entity_id: id.into(),
                            role_id: conf.config_id,
                            on_stage_without_control: false,
                        })
                        .collect(),
                    ..Default::default()
                }]),
                // vehicle_player_data: Vec::ne(),
                ..Default::default()
            }
        })
        .collect()
}

pub fn remove_entity(player: &Player, entity_id: i64, remove_type: ERemoveEntityType) {
    let mut world_ref = player.world.borrow_mut();
    let world = world_ref.get_mut_world_entity();

    if world.remove_entity(entity_id as i32) {
        player.notify(EntityRemoveNotify {
            remove_infos: vec![EntityRemoveInfo {
                entity_id,
                r#type: remove_type.into(),
            }],
            is_remove: true,
        });
    }
}

pub fn remove_entities(player: &Player, entities: &[&LevelEntityConfigData]) {
    let mut removed_entities = Vec::with_capacity(entities.len());
    // Enclose to drop borrow mut ASAP
    {
        let mut world_ref = player.world.borrow_mut();
        let world = world_ref.get_mut_world_entity();

        for entity in entities {
            let entity_id = entity.entity_id as i32; // TODO: Should be i64
            if world.remove_entity(entity_id) {
                removed_entities.push(world.get_entity_id(entity_id));
            }
        }
    }
    for entity_id in removed_entities {
        player.notify(EntityRemoveNotify {
            remove_infos: vec![EntityRemoveInfo {
                entity_id,
                r#type: 0,
            }],
            is_remove: true,
        });
    }
}

pub fn add_entities(player: &Player, entities: &[&LevelEntityConfigData], external_awake: bool) {
    let mut added_entities = Vec::with_capacity(entities.len());
    // Enclose to drop borrow mut ASAP
    {
        let mut world_ref = player.world.borrow_mut();
        let world = world_ref.get_mut_world_entity();

        for entity in entities {
            // Skip hidden entities
            if entity.is_hidden {
                tracing::debug!("Hidden entity with config id: {}", entity.entity_id);
                continue;
            }
            if entity.in_sleep && !external_awake {
                tracing::debug!(
                    "Sleep entity with config id not spawned: {}",
                    entity.entity_id
                );
                continue;
            }

            let blueprint_config = blueprint_config_data::get(&entity.blueprint_type);
            let template_config = template_config_data::get(&entity.blueprint_type);
            if blueprint_config.is_none() || template_config.is_none() {
                continue;
            }

            let entity_logic: EntityLogic = blueprint_config.unwrap().entity_logic;
            let (config_type, entity_type, mut entity_state) = match entity_logic {
                EntityLogic::Item => (
                    EntityConfigType::Level,
                    EEntityType::SceneItem,
                    EntityState::Default,
                ),
                EntityLogic::Animal => (
                    EntityConfigType::Level,
                    EEntityType::Animal,
                    EntityState::Default,
                ),
                EntityLogic::Monster => (
                    EntityConfigType::Level,
                    EEntityType::Monster,
                    EntityState::Born,
                ),
                EntityLogic::Vehicle => (
                    EntityConfigType::Level,
                    EEntityType::Vehicle,
                    EntityState::Default,
                ),
                EntityLogic::Npc => (
                    EntityConfigType::Level,
                    EEntityType::Npc,
                    EntityState::Default,
                ),
                EntityLogic::Vision => (
                    EntityConfigType::Level,
                    EEntityType::Vision,
                    EntityState::Default,
                ),
                EntityLogic::ClientOnly => (
                    EntityConfigType::Level,
                    EEntityType::ClientOnly,
                    EntityState::Default,
                ),
                EntityLogic::ServerOnly => {
                    tracing::debug!("Unhandled entity to be added of logic: {:?} with blueprint_type {} and id: {}", entity_logic, entity.blueprint_type, entity.entity_id);
                    continue;
                }
                EntityLogic::Custom => (
                    EntityConfigType::Level,
                    EEntityType::Custom,
                    EntityState::Default,
                ),
            };

            if entity.in_sleep {
                entity_state = EntityState::Sleep;
            }

            let config_id = entity.entity_id as i32; // TODO: i64????
            let map_id = entity.map_id;
            let components: ComponentsData = entity
                .components_data
                .merge_with_template(&template_config.unwrap().components_data);
            let tmp_entity = world.create_entity(config_id, config_type.into(), map_id);
            let mut builder = world.create_builder(tmp_entity);
            builder
                .with(ComponentContainer::EntityConfig(EntityConfig {
                    camp: components
                        .base_info_component
                        .as_ref()
                        .and_then(|b| b.camp)
                        .unwrap_or(0),
                    config_id,
                    config_type,
                    entity_type,
                    entity_state,
                }))
                .with(ComponentContainer::Position(Position(Transform::from(
                    &entity.transform[..],
                ))))
                .with(ComponentContainer::Visibility(Visibility {
                    is_visible: true,
                    is_actor_visible: true,
                }))
                // Some entities may not actually have movement, but it's okay since we won't
                // receive move package push for them
                .with(ComponentContainer::Movement(Movement::default()));

            build_autonomous_component(&mut builder, player.basic_info.id, entity_logic);
            build_interact_component(&mut builder, &components);
            build_tags_components(&mut builder, &components, player, blueprint_config.unwrap().entity_type, config_id as i64);
            build_attribute_component(&mut builder, &components, player.location.instance_id);
            build_ai_components(&mut builder, &components);
            added_entities.push(builder.build());
        }
    }

    let world_ref = player.world.borrow();
    let world = world_ref.get_world_entity();
    // Since kuro has issues, we can only send one
    for entity in added_entities {
        let mut pb = EntityPb {
            id: entity.entity_id as i64, // TODO: Should be i64
            ..Default::default()
        };

        world
            .get_entity_components(entity.entity_id)
            .into_iter()
            .for_each(|comp| comp.set_pb_data(&mut pb));

        player.notify(EntityAddNotify {
            entity_pbs: vec![pb],
            remove_tag_ids: true,
        });
    }
}

#[inline(always)]
fn build_autonomous_component(builder: &mut EntityBuilder, id: i32, logic: EntityLogic) {
    // TODO: Review if other types have autonomous
    match logic {
        EntityLogic::Item => {
            builder.with(ComponentContainer::Autonomous(Autonomous {
                autonomous_id: id,
            }));
        }
        EntityLogic::Animal => {}
        EntityLogic::Monster => {}
        EntityLogic::Vehicle => {}
        EntityLogic::Npc => {}
        EntityLogic::Vision => {}
        EntityLogic::ClientOnly => {}
        EntityLogic::ServerOnly => {}
        EntityLogic::Custom => {}
    };
}

#[inline(always)]
fn build_interact_component(builder: &mut EntityBuilder, components: &ComponentsData) {
    if components.interact_component.is_some() {
        builder.with(ComponentContainer::Interact(Interact {}));
    }
}

#[inline(always)]
fn build_tags_components(
    builder: &mut EntityBuilder,
    components: &ComponentsData,
    player: &Player,
    entity_type: EntityType,
    config_id: i64,
) {
    if let Some(entity_state_component) = &components.entity_state_component {
        let state = match entity_type {
            EntityType::Teleporter | EntityType::TemporaryTeleporter => {
                let result = player.teleports.teleports_data.iter()
                    .find(|teleporter| teleporter.entity_config_id == config_id);
                match result.is_some() {
                    true => tag_utils::get_tag_id_by_name("关卡.Common.状态.激活"),
                    false => tag_utils::get_tag_id_by_name("关卡.Common.状态.常态"),
                }
            }
            _ => {
                // TODO: how to get states???
                let unlocked = false;
                match unlocked {
                    true => tag_utils::get_tag_id_by_name("关卡.Common.状态.激活"),
                    false => tag_utils::get_tag_id_by_name("关卡.Common.状态.常态"),
                }
            }
        };
        builder
            .with(ComponentContainer::StateTag(StateTag {
                state_tag_id: state,
            }))
            .with(ComponentContainer::Tag(Tag {
                // TODO:
                gameplay_tags: vec![],
                entity_common_tags: vec![state],
                // TODO:
                init_gameplay_tag: false,
            }));
    }
}

#[inline(always)]
fn build_attribute_component(
    builder: &mut EntityBuilder,
    components: &ComponentsData,
    instance_id: i32,
) {
    if let Some(attribute_component) = &components.attribute_component {
        if attribute_component.disabled.unwrap_or_default() {
            return;
        }
        if let Some(property_id) = attribute_component.property_id {
            let inst_data = wicked_waifus_data::instance_dungeon_data::iter()
                .find(|d| d.id == instance_id)
                .unwrap();

            let mut level = inst_data.entity_level;
            if level == 0 {
                // TODO:
                //  - iterate area_data
                //  - find player.basic_info.area_id
                //  - get player world level
                //  - get area.world_monster_level_max
                level = 90;
            }
            builder.with(ComponentContainer::Attribute(Attribute::from_data(
                &get_monster_props_by_level(property_id, level),
                attribute_component.hardness_mode_id,
                attribute_component.rage_mode_id,
            )));
        }
    }
}

#[inline(always)]
fn build_ai_components(builder: &mut EntityBuilder, components: &ComponentsData) {
    if let Some(ai_component) = &components.ai_component {
        builder.with(ComponentContainer::MonsterAi(MonsterAi {
            weapon_id: ai_component
                .weapon_id
                .as_deref()
                .and_then(|id| id.parse().ok())
                .unwrap_or(0),
            hatred_group_id: 0,   // TODO:
            ai_team_init_id: 100, // TODO:
            combat_message_id: 0, // TODO:
        }));
        if let Some(ai_id) = ai_component.ai_id {
            builder.with(ComponentContainer::Fsm(Fsm::from_ai_id(ai_id)));
        }
    }
}
