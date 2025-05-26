use std::collections::HashSet;

use crate::logic::components::{Attribute, Equip, WeaponSkin};
use crate::logic::ecs::component::ComponentContainer;
use crate::logic::handler::attribute_changed_combat_notify;
use crate::logic::math::Weapon;
use crate::logic::player::Player;
use crate::{modify_component, query_components};
use wicked_waifus_protocol::{
    ArrayIntInt, EAttributeType, EntityEquipChangeNotify, EntityEquipSkinChangeNotify,
    EquipComponentPb, EquipTakeOnNotify, EquipTakeOnRequest, EquipTakeOnResponse,
    EquipWeaponSkinRequest, EquipWeaponSkinResponse, ErrorCode, GameplayAttributeData,
    LoadEquipData, PbRolePropsNotify, RoleLoadEquipData, SendEquipSkinRequest,
    SendEquipSkinResponse, WeaponSkinComponentPb, WeaponSkinDeleteNotify, WeaponSkinRequest,
    WeaponSkinResponse,
};

pub fn on_weapon_skin_request(
    player: &Player,
    _request: WeaponSkinRequest,
    response: &mut WeaponSkinResponse,
) {
    response.equip_list = player
        .role_list
        .values()
        .filter(|role| role.weapon_skin_id != 0)
        .map(|role| LoadEquipData {
            role_id: role.role_id,
            skin_id: role.weapon_skin_id,
        })
        .collect();

    response.error_code = ErrorCode::Success.into();
}

pub fn on_equip_weapon_skin_request(
    player: &mut Player,
    request: EquipWeaponSkinRequest,
    response: &mut EquipWeaponSkinResponse,
) {
    let Some(equip_data) = request.data else {
        return;
    };

    let role = player.role_list.get_mut(&equip_data.role_id);
    let Some(role) = role else {
        response.error_code = ErrorCode::NotValidRole.into();
        return;
    };

    // Verify Id exist in bindata
    let Some(skin_data) =
        wicked_waifus_data::weapon_skin_data::iter().find(|data| data.id == equip_data.skin_id)
    else {
        response.error_code = ErrorCode::WeaponSkinDataErr.into();
        return;
    };

    // Verify Skin is unlocked
    if !player.unlocked_skins.weapon_skins.contains(&skin_data.id) {
        response.error_code = ErrorCode::WeaponSkinUnLockErr.into();
        return;
    }

    role.weapon_skin_id = equip_data.skin_id;
    {
        let world_ref = player.world.borrow();
        let world = world_ref.get_world_entity();
        let entity_id = world.get_entity_id(equip_data.role_id);
        modify_component!(
            world.get_entity_components(entity_id as i32),
            WeaponSkin,
            |skin_component: &mut WeaponSkin| {
                skin_component.skin_id = equip_data.skin_id;
            }
        );
        player.notify(EntityEquipSkinChangeNotify {
            entity_id,
            weapon_skin_component_pb: Some(WeaponSkinComponentPb {
                weapon_skin_id: equip_data.skin_id,
            }),
        });
    }

    // Is the all list needed or only the new one??
    response.data_list = player
        .role_list
        .values()
        .filter(|role| role.weapon_skin_id != 0)
        .map(|role| LoadEquipData {
            role_id: role.role_id,
            skin_id: role.weapon_skin_id,
        })
        .collect();
    response.error_code = ErrorCode::Success.into();
}

pub fn on_send_equip_skin_request(
    player: &mut Player,
    request: SendEquipSkinRequest,
    response: &mut SendEquipSkinResponse,
) {
    let role = player.role_list.get_mut(&request.role_id);
    let Some(role) = role else {
        response.error_code = ErrorCode::NotValidRole.into();
        return;
    };

    let old_skin_id = role.weapon_skin_id;
    role.weapon_skin_id = 0;
    {
        let world_ref = player.world.borrow();
        let world = world_ref.get_world_entity();
        let entity_id = world.get_entity_id(request.role_id);
        modify_component!(
            world.get_entity_components(entity_id as i32),
            WeaponSkin,
            |skin_component: &mut WeaponSkin| {
                skin_component.skin_id = 0;
            }
        );
        player.notify(EntityEquipSkinChangeNotify {
            entity_id,
            weapon_skin_component_pb: Some(WeaponSkinComponentPb { weapon_skin_id: 0 }),
        });
        player.notify(WeaponSkinDeleteNotify {
            role_id: request.role_id,
            skin_id: old_skin_id,
        })
    }
    response.error_code = ErrorCode::Success.into();
}

pub fn on_equip_take_on_request(
    player: &mut Player,
    request: EquipTakeOnRequest,
    response: &mut EquipTakeOnResponse,
) {
    let Some(data) = request.data else {
        return;
    };

    let weapons = if let Ok(weapon) = player
        .inventory
        .swap_weapon(data.equip_inc_id, data.role_id)
    {
        weapon
    } else {
        response.error_code = ErrorCode::ErrItemNotFound.into();
        return;
    };

    let equip_data = weapons
        .iter()
        .map(|(incr, (_, weapon))| RoleLoadEquipData {
            role_id: weapon.role_id,
            pos: 0, // Check this field
            equip_inc_id: *incr,
        })
        .collect::<std::vec::Vec<_>>();

    player.notify(EquipTakeOnNotify {
        data_list: equip_data.clone(),
    });

    let mut updated_role_ids = HashSet::new();

    for (incr_id, (before_weapon, after_weapon)) in weapons {
        // Remove attributes from weapon before
        if before_weapon.role_id != 0 {
            let is_in_formation = player
                .formation_list
                .get(&player.cur_formation_id)
                .map_or(false, |f| f.role_ids.contains(&before_weapon.role_id));

            let Some(role) = player.role_list.get_mut(&before_weapon.role_id) else {
                response.error_code = ErrorCode::ErrItemNotFound.into();
                return;
            };

            let attr_removed = Weapon::remove_weapon_attributes(
                &mut role.base_prop,
                &mut role.add_prop,
                &after_weapon,
            );

            if is_in_formation {
                let world_ref = player.world.borrow();
                let world = world_ref.get_world_entity();
                let entity_id = world.get_entity_id(before_weapon.role_id);

                for (key, (value, is_ratio)) in attr_removed {
                    modify_component!(
                        world.get_entity_components(entity_id as i32),
                        Attribute,
                        |attribute: &mut Attribute| {
                            let e_attr_type = EAttributeType::try_from(key).unwrap();
                            if is_ratio {
                                if let Some(attr) = attribute.attr_map.get_mut(&e_attr_type) {
                                    attr.1 = value;
                                }
                            } else {
                                if let Some(attr) = attribute.attr_map.get_mut(&e_attr_type) {
                                    attr.0 = value;
                                }
                            }
                        }
                    );
                }
            }

            updated_role_ids.insert(before_weapon.role_id);
        }

        // Assign attributes to weapon after
        if after_weapon.role_id != 0 {
            let is_in_formation = player
                .formation_list
                .get(&player.cur_formation_id)
                .map_or(false, |f| f.role_ids.contains(&after_weapon.role_id));

            let Some(role) = player.role_list.get_mut(&after_weapon.role_id) else {
                response.error_code = ErrorCode::ErrItemNotFound.into();
                return;
            };

            let attr_added = Weapon::assign_weapon_attributes(
                &mut role.base_prop,
                &mut role.add_prop,
                &after_weapon,
            );

            if is_in_formation {
                let world_ref = player.world.borrow();
                let world = world_ref.get_world_entity();
                let entity_id = world.get_entity_id(after_weapon.role_id);

                if let Some((weapon_id, breach)) = player.inventory.get_weapon_equip_info(incr_id) {
                    role.equip_weapon = weapon_id;

                    modify_component!(
                        world.get_entity_components(entity_id as i32),
                        Equip,
                        |equip_component: &mut Equip| {
                            equip_component.weapon_id = weapon_id;
                            equip_component.weapon_breach_level = breach;
                        }
                    );

                    player.notify(EntityEquipChangeNotify {
                        entity_id,
                        equip_component: Some(EquipComponentPb {
                            weapon_id: weapon_id,
                            weapon_breach_level: breach,
                        }),
                    });
                }

                for (key, (value, is_ratio)) in attr_added {
                    modify_component!(
                        world.get_entity_components(entity_id as i32),
                        Attribute,
                        |attribute: &mut Attribute| {
                            let e_attr_type = EAttributeType::try_from(key).unwrap();
                            if is_ratio {
                                if let Some(attr) = attribute.attr_map.get_mut(&e_attr_type) {
                                    attr.1 = value;
                                }
                            } else {
                                if let Some(attr) = attribute.attr_map.get_mut(&e_attr_type) {
                                    attr.0 = value;
                                }
                            }
                        }
                    );
                }
            }
            updated_role_ids.insert(after_weapon.role_id);
        }
    }

    for role_id in updated_role_ids {
        let world_ref = player.world.borrow();
        let world = world_ref.get_world_entity();
        let entity_id = world.get_entity_id(role_id);

        if let Some(role) = player.role_list.get(&role_id) {
            if entity_id != -1 {
                if let Some(attribute) = query_components!(world, entity_id, Attribute).0 {
                    let attributes = attribute
                        .attr_map
                        .iter()
                        .map(|(ty, (base, incr))| GameplayAttributeData {
                            attribute_type: (*ty).into(),
                            current_value: *base,
                            value_increment: *base + *incr,
                        })
                        .collect::<std::vec::Vec<_>>();

                    attribute_changed_combat_notify(player, entity_id, attributes);
                }
            } else {
                player.notify(PbRolePropsNotify {
                    role_id,
                    base_prop: role
                        .base_prop
                        .iter()
                        .map(|(&k, &v)| ArrayIntInt { key: k, value: v })
                        .collect(),
                    add_prop: role
                        .add_prop
                        .iter()
                        .map(|(&k, &v)| ArrayIntInt { key: k, value: v })
                        .collect(),
                });
            }
        }
    }

    response.data_list = equip_data;
    response.error_code = ErrorCode::Success.into();
}
