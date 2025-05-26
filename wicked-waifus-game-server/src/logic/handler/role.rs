use std::collections::HashSet;

use crate::logic::components::{ParaglidingSkin, RoleSkin, SoarWingSkin, WeaponSkin};
use crate::logic::ecs::component::ComponentContainer;
use crate::logic::player::{ItemUsage, Player};
use crate::logic::role::{Role, RoleFormation};
use crate::modify_component;
use wicked_waifus_protocol::{ArrayIntInt, ClientCurrentRoleReportRequest, ClientCurrentRoleReportResponse, ERemoveEntityType, EntityAddNotify, EntityEquipSkinChangeNotify, EntityFlySkinChangeData, EntityPb, EntityRemoveInfo, EntityRemoveNotify, EquipFlySkinData, ErrorCode, FlySkinConfigData, FlySkinWearAllRoleRequest, FlySkinWearAllRoleResponse, FlySkinWearRequest, FlySkinWearResponse, FormationAttrRequest, FormationAttrResponse, PbUpLevelRoleRequest, PbUpLevelRoleResponse, PlayerMotionRequest, PlayerMotionResponse, RoleBreakThroughViewRequest, RoleBreakThroughViewResponse, RoleFavorListRequest, RoleFavorListResponse, RoleFlyEquipChangeNotify, RoleLevelUpViewRequest, RoleLevelUpViewResponse, RoleShowListUpdateRequest, RoleShowListUpdateResponse, RoleSkinChangeRequest, RoleSkinChangeResponse, SoarWingOrParaglidingSkinChangeNotify, UnlockRoleSkinListRequest, UnlockRoleSkinListResponse, UpdateFormationRequest, UpdateFormationResponse, WeaponSkinComponentPb};

pub fn on_role_show_list_update_request(
    player: &mut Player,
    request: RoleShowListUpdateRequest,
    response: &mut RoleShowListUpdateResponse,
) {
    let role_ids: HashSet<i32> = player.role_list.keys().cloned().collect();
    let all_exist = request.role_list.iter().all(|id| role_ids.contains(id));

    if all_exist {
        player.basic_info.role_show_list = request.role_list;
        response.error_code = ErrorCode::Success.into();
    } else {
        response.error_code = ErrorCode::InvalidRequest.into(); // TODO: replace with appropriate error code
    }
}

pub fn on_client_current_role_report_request(
    _player: &Player,
    request: ClientCurrentRoleReportRequest,
    response: &mut ClientCurrentRoleReportResponse,
) {
    response.current_entity_id = request.current_entity_id;
    response.player_id = request.player_id;
}

pub fn on_role_favor_list_request(
    _player: &Player,
    _request: RoleFavorListRequest,
    response: &mut RoleFavorListResponse,
) {
    response.favor_list = vec![]; // TODO: add favor
    response.error_code = ErrorCode::Success.into();
}

pub fn on_formation_attr_request(
    _player: &Player,
    _request: FormationAttrRequest,
    response: &mut FormationAttrResponse,
) {
    response.error_code = ErrorCode::Success.into();
}

pub fn on_update_formation_request(
    player: &mut Player,
    request: UpdateFormationRequest,
    response: &mut UpdateFormationResponse,
) {
    let mut world_ref = player.world.borrow_mut();
    let world = world_ref.get_mut_world_entity();

    for formation in request.formations {
        let formation_id = formation.formation_id;
        let cur_role = formation.cur_role;
        let is_current = formation.is_current;

        if is_current {
            // update player current formation id
            player.cur_formation_id = formation_id;

            // search old formation id and set real_formation_id, set is_current to false
            let mut real_formation_id = formation_id;
            if let Some(rf) = player
                .formation_list
                .values_mut()
                .find(|rf| rf.is_current && rf.id != formation_id)
            {
                real_formation_id = rf.id;
                rf.is_current = false;
            }

            if let Some(old_formation) = player.formation_list.get(&real_formation_id) {
                let removed_entities: Vec<i64> = old_formation
                    .role_ids
                    .iter()
                    .map(|&role_id| world.get_entity_id(role_id))
                    .collect();
                removed_entities.iter().for_each(|&entity_id| {
                    world.remove_entity(entity_id as i32);
                });
                player.notify(player.build_player_entity_remove_notify(
                    removed_entities,
                    ERemoveEntityType::RemoveTypeNormal,
                ));
            }

            let added_roles: Vec<Role> = formation
                .role_ids
                .iter()
                .map(|&role_id| Role::new(role_id))
                .collect();

            if !added_roles.is_empty() {
                // add new roles
                player.notify(player.build_player_entity_add_notify(added_roles, world));
            }

            // send update group formation notify
            player.notify(player.build_update_group_formation_notify(
                RoleFormation {
                    id: formation_id,
                    cur_role,
                    role_ids: formation.role_ids.clone(),
                    is_current,
                },
                world,
            ));

            response.formation = Some(formation.clone());
        }

        // update all formation and check formation_list
        player
            .formation_list
            .entry(formation_id)
            .and_modify(|r| {
                r.cur_role = formation.cur_role;
                r.role_ids = formation.role_ids.clone();
                r.is_current = is_current;
            })
            .or_insert(RoleFormation {
                id: formation_id,
                cur_role: formation.cur_role,
                role_ids: formation.role_ids,
                is_current,
            });
    }

    player.notify(player.build_update_formation_notify());

    response.error_code = ErrorCode::Success.into();
}

pub fn on_player_motion_request(
    _: &Player,
    request: PlayerMotionRequest,
    response: &mut PlayerMotionResponse,
) {
    match wicked_waifus_data::motion_data::iter().find(|&motion| motion.id == request.motion) {
        None => response.error_id = ErrorCode::UnKnownError.into(),
        Some(_) => response.error_id = ErrorCode::Success.into(),
    }
}

pub fn on_unlock_role_skin_list_request(
    player: &Player,
    _request: UnlockRoleSkinListRequest,
    response: &mut UnlockRoleSkinListResponse,
) {
    response.role_skin_list = player.unlocked_skins.role_skins.iter().cloned().collect();
}

pub fn on_role_skin_change_request(
    player: &mut Player,
    request: RoleSkinChangeRequest,
    response: &mut RoleSkinChangeResponse,
) {
    // TODO: Should we verify role id first against bindata?
    let role = player.role_list.get_mut(&request.role_id);
    let Some(role) = role else {
        response.error_code = ErrorCode::NotValidRole.into();
        return;
    };

    // Verify Id exist in bindata
    let Some(skin_data) =
        wicked_waifus_data::role_skin_data::iter().find(|data| data.id == request.skin_id)
    else {
        response.error_code = ErrorCode::ErrRoleSkinConfig.into();
        return;
    };

    // Verify Skin is unlocked
    if !player.unlocked_skins.role_skins.contains(&skin_data.id) {
        response.error_code = ErrorCode::ErrRoleSkinLocked.into();
        return;
    }

    // Check that role has this skin
    if skin_data.role_id != role.role_id {
        response.error_code = ErrorCode::ErrRoleSkinNotMatch.into();
        return;
    }

    role.skin_id = request.skin_id;
    if request.is_wear_weapon_skin {
        if skin_data.suit_weapon_skin_id == 0 {
            response.error_code = ErrorCode::ErrRoleSkinWeaponNotSuit.into();
            return;
        }
        role.weapon_skin_id = skin_data.suit_weapon_skin_id;
    }
    {
        let world_ref = player.world.borrow();
        let world = world_ref.get_world_entity();
        let entity_id = world.get_entity_id(request.role_id);
        modify_component!(
            world.get_entity_components(entity_id as i32),
            RoleSkin,
            |skin_component: &mut RoleSkin| {
                skin_component.skin_id = role.skin_id;
            }
        );
        if request.is_wear_weapon_skin {
            // Check for suit_weapon_skin_id == 0 has already been done
            modify_component!(
                world.get_entity_components(entity_id as i32),
                WeaponSkin,
                |skin_component: &mut WeaponSkin| {
                    skin_component.skin_id = skin_data.suit_weapon_skin_id;
                }
            );
            // Since the whole entity is recreated this shouldn't be needed but meh, whatever
            player.notify(EntityEquipSkinChangeNotify {
                entity_id,
                weapon_skin_component_pb: Some(WeaponSkinComponentPb {
                    weapon_skin_id:skin_data.suit_weapon_skin_id,
                }),
            });
        }
        player.notify(EntityRemoveNotify {
            remove_infos: vec![EntityRemoveInfo {
                entity_id,
                r#type: 0,
            }],
            is_remove: false,
        });

        let mut pb = EntityPb {
            id: entity_id,
            ..Default::default()
        };

        world
            .get_entity_components(entity_id as i32)
            .into_iter()
            .for_each(|comp| comp.set_pb_data(&mut pb));

        player.notify(EntityAddNotify {
            entity_pbs: vec![pb],
            remove_tag_ids: false,
        });
    }
    player.notify(player.build_update_formation_notify());
    response.error_code = ErrorCode::Success.into();
}

pub fn on_fly_skin_wear_request(
    player: &mut Player,
    request: FlySkinWearRequest,
    response: &mut FlySkinWearResponse,
) {
    let role = player.role_list.get_mut(&request.role_id);
    let Some(role) = role else {
        response.error_code = ErrorCode::NotValidRole.into();
        return;
    };

    // Verify Id exist in bindata
    let skin =
        wicked_waifus_data::fly_skin_config_data::iter().find(|&skin| skin.id == request.skin_id);
    let Some(skin) = skin else {
        response.error_code = ErrorCode::NoFlySkinItem.into();
        return;
    };

    match skin.skin_type {
        0 => {
            // Verify Skin is unlocked
            if !player.unlocked_skins.fly_skins.contains(&skin.id) {
                response.error_code = ErrorCode::ErrRoleSkinLocked.into();
                return;
            }
            role.fly_skin_id = request.skin_id
        }
        1 => {
            if !player.unlocked_skins.wing_skins.contains(&skin.id) {
                response.error_code = ErrorCode::ErrRoleSkinLocked.into();
                return;
            }
            role.wing_skin_id = request.skin_id
        }
        _ => {
            response.error_code = ErrorCode::FlySkinTypeErr.into();
            return;
        }
    }
    {
        let world_ref = player.world.borrow();
        let world = world_ref.get_world_entity();
        let entity_id = world.get_entity_id(request.role_id);
        match skin.skin_type {
            0 => {
                modify_component!(
                    world.get_entity_components(entity_id as i32),
                    SoarWingSkin,
                    |skin_component: &mut SoarWingSkin| {
                        skin_component.skin_id = role.skin_id;
                    }
                );
            }
            1 => {
                modify_component!(
                    world.get_entity_components(entity_id as i32),
                    ParaglidingSkin,
                    |skin_component: &mut ParaglidingSkin| {
                        skin_component.skin_id = role.skin_id;
                    }
                );
            }
            _ => unreachable!("Already tested above"),
        }
        player.notify(SoarWingOrParaglidingSkinChangeNotify {
            fly_skin_data: vec![EntityFlySkinChangeData {
                entity_id,
                fly_skin_config_data: vec![FlySkinConfigData {
                    skin_id: request.skin_id,
                    fly_skin_id: skin.skin_type,
                }],
            }],
        });
    }

    player.notify(RoleFlyEquipChangeNotify {
        fly_skin_data: vec![EquipFlySkinData {
            role_id: request.role_id,
            skin_id: request.skin_id,
        }],
    });
    response.error_code = ErrorCode::Success.into();
}

pub fn on_fly_skin_wear_all_role_request(
    player: &mut Player,
    request: FlySkinWearAllRoleRequest,
    response: &mut FlySkinWearAllRoleResponse,
) {
    let skin =
        wicked_waifus_data::fly_skin_config_data::iter().find(|&skin| skin.id == request.skin_id);
    let Some(skin) = skin else {
        response.error_code = ErrorCode::NoFlySkinItem.into();
        return;
    };

    match skin.skin_type {
        0 => {
            // Verify Skin is unlocked
            if !player.unlocked_skins.fly_skins.contains(&skin.id) {
                response.error_code = ErrorCode::ErrRoleSkinLocked.into();
                return;
            }
            for role in player.role_list.values_mut() {
                role.fly_skin_id = request.skin_id;
            }
        }
        1 => {
            if !player.unlocked_skins.wing_skins.contains(&skin.id) {
                response.error_code = ErrorCode::ErrRoleSkinLocked.into();
                return;
            }
            for role in player.role_list.values_mut() {
                role.wing_skin_id = request.skin_id;
            }
        }
        _ => {
            response.error_code = ErrorCode::FlySkinTypeErr.into();
            return;
        }
    }
    player.notify(RoleFlyEquipChangeNotify {
        fly_skin_data: player
            .role_list
            .values()
            .map(|r| EquipFlySkinData {
                role_id: r.role_id,
                skin_id: request.skin_id,
            })
            .collect::<Vec<_>>(),
    });
    {
        let world_ref = player.world.borrow();
        let world = world_ref.get_world_entity();
        let data = player
            .role_list
            .values()
            .filter_map(|role| {
                let entity_id = world.get_entity_id(role.role_id);
                if entity_id == -1 {
                    None
                } else {
                    match skin.skin_type {
                        0 => {
                            modify_component!(
                                world.get_entity_components(entity_id as i32),
                                SoarWingSkin,
                                |skin_component: &mut SoarWingSkin| {
                                    skin_component.skin_id = role.skin_id;
                                }
                            );
                        }
                        1 => {
                            modify_component!(
                                world.get_entity_components(entity_id as i32),
                                ParaglidingSkin,
                                |skin_component: &mut ParaglidingSkin| {
                                    skin_component.skin_id = role.skin_id;
                                }
                            );
                        }
                        _ => unreachable!("Already tested above"),
                    }
                    Some(EntityFlySkinChangeData {
                        entity_id,
                        fly_skin_config_data: vec![FlySkinConfigData {
                            skin_id: request.skin_id,
                            fly_skin_id: skin.skin_type,
                        }],
                    })
                }
            })
            .collect::<Vec<_>>();
        player.notify(SoarWingOrParaglidingSkinChangeNotify {
            fly_skin_data: data,
        });
    }
    response.error_code = ErrorCode::Success.into();
}

pub fn on_role_level_up_view_request(
    player: &mut Player,
    request: RoleLevelUpViewRequest,
    response: &mut RoleLevelUpViewResponse,
) {
    let role = player.role_list.get(&request.role_id);
    let Some(role) = role else {
        response.error_code = ErrorCode::NotValidRole.into();
        return;
    };

    response.level = role.level;
    // TODO: shall we get from data? seems client can do it by himself
    response.level_exp_info = vec![ArrayIntInt { key: 1, value: 200 }];
    response.exp = role.exp;
    // it seems add_exp, final_prop, cost_list amd overflow_list are handled by client, so skip

    let items = wicked_waifus_data::role_exp_item_data::iter()
        .map(|(&id, _)| id)
        .collect::<Vec<_>>();
    response.item_list = player.inventory.to_array_int_int_filtered(&items);
    response.error_code = ErrorCode::Success.into();
}

pub fn on_pb_up_level_role_request(
    player: &mut Player,
    request: PbUpLevelRoleRequest,
    response: &mut PbUpLevelRoleResponse,
) {
    response.role_id = request.role_id;
    let role = player.role_list.get(&request.role_id);
    let Some(role) = role else {
        response.error_code = ErrorCode::NotValidRole.into();
        return;
    };

    // TODO: no shell_credit??? :turtle_skull:
    let items = player.inventory.consume_items(
        &request
            .item_list
            .iter()
            .map(|item| ItemUsage {
                id: item.key,
                quantity: -item.value,
            })
            .collect::<Vec<_>>(),
    );
    let Ok(_items) = items else {
        response.error_code = ErrorCode::ErrConsumeNotEnough.into();
        return;
    };

    let mut total_exp = request
        .item_list
        .iter()
        .filter_map(|item| {
            wicked_waifus_data::role_exp_item_data::get(&item.key)
                .map(|exp| exp.basic_exp * item.value)
        })
        .sum();

    let role_level_consume_id = wicked_waifus_data::role_info_data::iter()
        .find(|role| role.id == request.role_id)
        .map(|role| role.level_consume_id)
        .unwrap_or(10001);

    let mut levels_consume = wicked_waifus_data::role_level_consume_data::iter()
        .filter(|consume| {
            consume.consume_group_id == role_level_consume_id && consume.level > role.level
        }) // TODO: add upper bound too(till breakthrough)
        .collect::<Vec<_>>();
    levels_consume.sort_by_key(|item| item.level);

    let mut level = role.level;
    for level_consume in levels_consume {
        if level_consume.exp_count > total_exp {
            break;
        }
        total_exp -= level_consume.exp_count;
        level = level_consume.level;
    }
    response.level = level;
    response.exp = total_exp;

    // TODO is item_map the overflowing items or all items used? also should we sent more notifies?
    response.error_code = ErrorCode::Success.into();
}

// on_role_break_through_view_request

pub fn on_role_break_through_view_request(
    player: &mut Player,
    request: RoleBreakThroughViewRequest,
    response: &mut RoleBreakThroughViewResponse,
) {
    let role = player.role_list.get(&request.role_id);
    let Some(role) = role else {
        response.error_code = ErrorCode::NotValidRole.into();
        return;
    };

    // TODO:
    // if !condition {
    //     response.error_code = ErrorCode::ErrRoleConditionNotFind.into();
    //     response.is_condition_finish = false;
    // }

    let role_breach_id = wicked_waifus_data::role_info_data::iter()
        .find(|role| role.id == request.role_id)
        .map(|role| role.breach_id)
        .unwrap_or(request.role_id);

    let condition = wicked_waifus_data::role_breach_data::iter()
        .find(|role_breach_data| {
            role_breach_data.breach_group_id == role_breach_id
                && role_breach_data.breach_level == role.breakthrough
        })
        .unwrap(); // TODO: handling

    response.error_code = ErrorCode::Success.into();
    response.level_limit = condition.max_level;

    response.cost_list = condition
        .breach_consume
        .iter()
        .map(|(&id, &count)| ArrayIntInt {
            key: id,
            value: count,
        })
        .collect::<Vec<_>>();
    // TODO: un_lock_skill_id, reward_list, final_prop
    response.is_condition_finish = true; // is this last??
}
