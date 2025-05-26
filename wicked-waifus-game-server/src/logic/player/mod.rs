pub use in_world_player::InWorldPlayer;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use wicked_waifus_commons::time_util;
use wicked_waifus_data::motion_data;
use wicked_waifus_protocol::message::Message;
use wicked_waifus_protocol::player_attr::Value;
use wicked_waifus_protocol::{
    AdventreTask, AdventureManualData, AdventureUpdateNotify, AdviceSettingNotify, BuffItemNotify,
    ControlInfoNotify, EEntityType, ERemoveEntityType, EnergyInfo, EnergyUpdateNotify,
    EntityAddNotify, EntityConfigType, EntityPb, EntityRemoveInfo, EntityRemoveNotify, EntityState,
    FavorItem, FightFormationNotifyInfo, FightRoleInfo, FightRoleInfos, FlyEquipAddNotify,
    FlySkinEquipData, GroupFormation, HostTeleportUnlockNotify, InstDataNotify, ItemPkgOpenNotify,
    LevelPlayInfoNotify, LivingStatus, MailInfosNotify, MonthCardDailyRewardNotify,
    MoonChasingTargetGetCountNotify, MoonChasingTrackMoonHandbookRewardNotify,
    NormalItemUpdateNotify, PassiveSkillNotify, PbGetRoleListNotify, PlayerAttr, PlayerAttrKey,
    PlayerAttrNotify, PlayerAttrType, PlayerFightFormations, PlayerVarNotify, ProtocolUnit,
    PushContextIdNotify, PushDataCompleteNotify, RoguelikeCurrencyNotify, RoleChangeUnlockNotify,
    RoleFavor, RoleFavorListNotify, RoleFlyEquipNotify, RoleMotion, RoleMotionListNotify,
    SettingNotify, TeleportUpdateNotify, UnlockSkinDataNotify, UpdateFormationNotify,
    UpdateGroupFormationNotify,
};
use wicked_waifus_protocol_internal::{PlayerBasicData, PlayerRoleData, PlayerSaveData};

use super::{
    ecs::world::World,
    role::{Role, RoleFormation},
};
use crate::logic::components::RoleSkin;
use crate::logic::ecs::world::WorldEntity;
use crate::logic::player::basic_info::PlayerBasicInfo;
use crate::logic::player::explore_tools::ExploreTools;
use crate::logic::player::location::PlayerLocation;
use crate::logic::player::player_adventure::PlayerAdventureStatus;
use crate::logic::player::player_advice::PlayerAdviceConfig;
use crate::logic::player::player_chat::PlayerChat;
use crate::logic::player::player_func::PlayerFunc;
use crate::logic::player::player_guides::PlayerGuides;
pub use crate::logic::player::player_inventory::ItemUsage;
use crate::logic::player::player_inventory::PlayerInventory;
use crate::logic::player::player_map_trace::PlayerMapTrace;
pub use crate::logic::player::player_mc_element::Element;
use crate::logic::player::player_mc_element::PlayerMcElement;
use crate::logic::player::player_month_card::PlayerMonthCard;
use crate::logic::player::player_teleports::{PlayerTeleport, PlayerTeleports};
use crate::logic::player::player_tutorials::{PlayerTutorial, PlayerTutorials};
use crate::logic::player::Element::Spectro;
use crate::logic::{
    components::{
        Attribute, EntityConfig, Equip, FightBuff, Movement, OwnerPlayer, ParaglidingSkin,
        PlayerOwnedEntityMarker, Position, SoarWingSkin, Visibility, VisionSkill, WeaponSkin,
    },
    ecs::component::ComponentContainer,
};
use crate::session::Session;
use crate::{config, create_player_entity_pb};
use crate::logic::player::player_unlocked_skins::PlayerUnlockedSkins;

mod basic_info;
mod explore_tools;
mod in_world_player;
mod location;
mod player_adventure;
mod player_advice;
mod player_chat;
mod player_func;
mod player_guides;
mod player_inventory;
mod player_map_trace;
mod player_mc_element;
mod player_month_card;
mod player_teleports;
mod player_tutorials;
mod player_unlocked_skins;

pub struct Player {
    session: Option<Arc<Session>>,
    // Persistent
    pub basic_info: PlayerBasicInfo,
    pub role_list: HashMap<i32, Role>,
    pub formation_list: HashMap<i32, RoleFormation>,
    pub cur_formation_id: i32,
    pub location: PlayerLocation,
    pub func: PlayerFunc,
    pub explore_tools: ExploreTools,
    pub player_chat: PlayerChat,
    pub guides: PlayerGuides,
    pub advise: PlayerAdviceConfig,
    pub adventure_status: PlayerAdventureStatus,
    pub inventory: PlayerInventory,
    pub teleports: PlayerTeleports,
    pub tutorials: PlayerTutorials,
    pub map_trace: PlayerMapTrace,
    pub month_card: PlayerMonthCard,
    pub mc_element: PlayerMcElement,
    pub unlocked_skins: PlayerUnlockedSkins,
    // Runtime
    pub world: Rc<RefCell<World>>,
    pub last_save_time: u64,
    pub quadrant_id: u64,
}

impl Player {
    pub fn init(&mut self) {
        if self.role_list.is_empty() || self.formation_list.is_empty() {
            self.init_role_and_formation();
        }

        self.ensure_basic_unlock_func();
    }

    pub fn notify_general_data(&self) {
        self.notify(self.build_adventure_task_notify());
        self.notify(AdviceSettingNotify {
            is_show: self.advise.is_show,
        });
        self.notify(self.basic_info.build_notify(&self.inventory));
        // CalabashMsgNotify + CalabashLevelRewardsNotify
        self.notify(ControlInfoNotify {
            forbid_list: vec![], // Disable function prohibition
        });
        self.notify(self.explore_tools.build_explore_tool_all_notify());
        self.notify(self.explore_tools.build_vision_explore_skill_notify());
        self.notify(self.explore_tools.build_roulette_update_notify());
        self.notify(self.build_role_favor_list_notify());
        self.notify(self.func.build_func_open_notify());
        self.notify(self.build_weapon_skin_notify());
        self.notify(self.build_fly_equip_notify());
        self.notify(self.build_role_fly_equip_notify());

        self.notify(InstDataNotify {
            enter_infos: vec![], // TODO: No effect in normal world, to implement for dungeon::logic()
        });
        self.notify(ItemPkgOpenNotify {
            open_pkg: (0..8).collect(),
        });
        self.notify(RoleChangeUnlockNotify {
            // TODO: element persistance
            unlock_role_ids: Role::get_mc_unlock_variations(vec![
                Element::Spectro,
                Element::Havoc,
                Element::Aero,
            ]),
            next_allow_change_time: 0,
        });
        self.notify(self.build_role_list_notify());
        // TODO
        self.notify(BuffItemNotify {
            item_buff_list: vec![],
            cr_1: vec![],
        });
        // TODO:
        self.notify(EnergyUpdateNotify {
            update_info: vec![
                EnergyInfo {
                    energy_count: 240,
                    last_renew_energy_time: 0,
                    energy_type: 5,
                },
                EnergyInfo {
                    energy_count: 480,
                    last_renew_energy_time: 0,
                    energy_type: 6,
                },
            ],
        });
        // TODO:
        self.notify(LevelPlayInfoNotify {
            level_play_info: vec![],
        });
        // PayShopInfoNotify
        // TODO:
        self.notify(PlayerVarNotify {
            var_infos: Default::default(),
        });
        self.notify(RoguelikeCurrencyNotify {});
        self.notify(self.build_motion_list_notify());
        self.notify(PassiveSkillNotify {
            role_passive_skill_info_list: vec![], // TODO:
        });

        // TODO:
        self.notify(MailInfosNotify { mail_infos: vec![] });
        // TODO:
        self.notify(SettingNotify {
            mobile_button_settings: vec![],
        });
        // TODO:
        self.notify(MoonChasingTrackMoonHandbookRewardNotify { ids: vec![] });
        // TODO:
        self.notify(MoonChasingTargetGetCountNotify {
            target_get_count: 0,
        });
        // TODO: [WWPS-1] Real implementation should fetch completed / uncompleted from db, lets return completed
        // for i in 0..80 {
        //     self.notify(MapUnlockFieldNotify { field_id: i });
        // }
        self.notify(PushContextIdNotify { id: 0 });
        self.notify(PushDataCompleteNotify {});
    }

    fn init_role_and_formation(&mut self) {
        self.role_list.clear();
        let mut role = Role::get_mc_based_on_sex(self.basic_info.sex, Spectro);
        role.name = self.basic_info.name.clone();
        self.role_list.insert(role.role_id, role);

        if config::get_config().default_unlocks.unlock_all_roles {
            Role::get_all_roles_except_mc().iter().for_each(|&role_id| {
                if !self.role_list.keys().any(|&k| k == role_id) {
                    self.role_list.insert(role_id, Role::new(role_id));
                }
            });
        } else {
            RoleFormation::default_roles().iter().for_each(|&role_id| {
                self.role_list.insert(role_id, Role::new(role_id));
            });
        }
        for role in self.role_list.values() {
            self.inventory
                .add_weapon(role.equip_weapon, 0, 1, 0, 0, 0, role.role_id)
                .unwrap();
        }

        self.formation_list.insert(1, RoleFormation::default());
        self.cur_formation_id = 1;

        self.formation_list.values_mut().for_each(|formation| {
            if formation.is_current && formation.id != 1 {
                formation.is_current = false;
            }
        });

        self.ensure_current_formation();
    }

    // Ensure basic functionality is unlocked
    // Should be handled by quest progression,
    // but as of right now, just unlock what we need
    fn ensure_basic_unlock_func(&mut self) {
        self.func.unlock(10026); // explore tools
    }

    fn ensure_current_formation(&mut self) {
        // If the list off formation is empty, add a default formation
        if self.formation_list.is_empty() {
            let mut role_list_clone = self.role_list.iter().clone();

            self.formation_list.insert(
                1,
                RoleFormation {
                    id: 1,
                    cur_role: role_list_clone.next().unwrap().1.role_id,
                    role_ids: role_list_clone
                        .take(3)
                        .map(|(&role_id, _)| role_id)
                        .collect(),
                    is_current: true,
                },
            );
        }

        // If there is no current formation, set the first formation as the current formation
        if !self.formation_list.values().any(|rf| rf.is_current) {
            self.formation_list.get_mut(&1).unwrap().is_current = true;
        }

        // Ensure that the set of character IDs for the current formation is not empty and that the current character ID is in the set
        if let Some(rf) = self.formation_list.values_mut().find(|rf| rf.is_current) {
            if rf.role_ids.is_empty() {
                rf.role_ids
                    .push(self.role_list.iter().next().unwrap().1.role_id);
            }

            if !rf.role_ids.contains(&rf.cur_role) {
                rf.cur_role = *rf.role_ids.iter().nth(0).unwrap();
            }
        }
    }

    pub fn build_in_world_player(&self) -> InWorldPlayer {
        InWorldPlayer {
            player_id: self.basic_info.id,
            player_name: self.basic_info.name.clone(),
            player_icon: 0,
            level: self.basic_info.level,
            group_type: 1,
        }
    }

    pub fn build_adventure_task_notify(&self) -> AdventureUpdateNotify {
        AdventureUpdateNotify {
            adventure_manual_data: self
                .adventure_status
                .status
                .iter()
                .map(|global_status| AdventureManualData {
                    adventre_task: global_status
                        .adventure_task_status
                        .iter()
                        .map(|status| AdventreTask {
                            id: status.id,
                            state: status.state,
                            adventre_progress: status.progress,
                        })
                        .collect::<Vec<_>>(),
                    now_chapter: global_status.now_chapter,
                    received_chapter: global_status.received_chapter,
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn build_role_favor_list_notify(&self) -> RoleFavorListNotify {
        RoleFavorListNotify {
            favor_list: self
                .role_list
                .values()
                .map(|role| RoleFavor {
                    role_id: role.role_id,
                    level: role.favor_level,
                    exp: role.favor_exp,
                    word_ids: wicked_waifus_data::favor_word_data::iter()
                        .filter(|&word| word.role_id == role.role_id && word.cond_group_id == 0) // TODO: handle conditions
                        .map(|word| FavorItem {
                            id: word.id,
                            status: 2,
                        })
                        .collect(),
                    story_ids: wicked_waifus_data::favor_story_data::iter()
                        .filter(|&story| story.role_id == role.role_id && story.cond_group_id == 0) // TODO: handle conditions
                        .map(|story| FavorItem {
                            id: story.id,
                            status: 2,
                        })
                        .collect(),
                    goods_ids: wicked_waifus_data::favor_goods_data::iter()
                        .filter(|&goods| goods.role_id == role.role_id && goods.cond_group_id == 0) // TODO: handle conditions
                        .map(|goods| FavorItem {
                            id: goods.id,
                            status: 2,
                        })
                        .collect(),
                    favor_quest: None, // TODO:
                })
                .collect(),
            role_condition_info_map: Default::default(),
        }
    }

    pub fn build_weapon_skin_notify(&self) -> UnlockSkinDataNotify {
        UnlockSkinDataNotify {
            phantom_skin_list: self.unlocked_skins.weapon_skins.iter().cloned().collect(),
            is_login: true,
        }
    }

    pub fn build_fly_equip_notify(&self) -> FlyEquipAddNotify {
        FlyEquipAddNotify {
            unlock_fly_skin_ids: self
                .unlocked_skins
                .fly_skins
                .iter()
                .chain(&self.unlocked_skins.wing_skins)
                .cloned()
                .collect(),
        }
    }

    pub fn build_role_fly_equip_notify(&self) -> RoleFlyEquipNotify {
        let merged: Vec<_> = self
            .unlocked_skins
            .fly_skins
            .iter()
            .chain(&self.unlocked_skins.wing_skins)
            .cloned()
            .collect();

        let mut equipped_skins: HashMap<i32, Vec<i32>> = HashMap::new();
        for role in self.role_list.values() {
            if role.fly_skin_id != 0 {
                equipped_skins
                    .entry(role.fly_skin_id)
                    .or_default()
                    .push(role.role_id);
            }
            if role.wing_skin_id != 0 {
                equipped_skins
                    .entry(role.wing_skin_id)
                    .or_default()
                    .push(role.role_id);
            }
        }

        RoleFlyEquipNotify {
            fly_skin_equip_data: merged
                .iter()
                .map(|&skin| match equipped_skins.get(&skin) {
                    Some(role_list) => FlySkinEquipData {
                        role_ids: role_list.to_vec(),
                        skin_id: skin,
                    },
                    None => FlySkinEquipData {
                        role_ids: vec![],
                        skin_id: skin,
                    },
                })
                .collect(),
        }
    }

    pub fn build_motion_list_notify(&self) -> RoleMotionListNotify {
        RoleMotionListNotify {
            motion_list: self
                .role_list
                .values()
                .map(|role| {
                    RoleMotion {
                        role_id: role.role_id,
                        motion_ids: motion_data::iter()
                            .filter(|motion| {
                                role.role_id == motion.role_id && motion.cond_group_id == 0
                            }) // TODO: handle conditions
                            .map(|motion| FavorItem {
                                id: motion.id,
                                status: 2,
                            })
                            .collect::<Vec<_>>(),
                    }
                })
                .collect::<Vec<_>>(),
            role_condition_info_map: Default::default(),
        }
    }

    pub fn build_player_entity_add_notify(&self, role_list: Vec<Role>, world: &mut WorldEntity) -> EntityAddNotify {
        create_player_entity_pb!(
            role_list,
            self.basic_info.cur_map_id,
            self,
            self.basic_info.id,
            self.location.position.clone(),
            self.explore_tools,
            world
        )
    }

    pub fn build_player_entity_remove_notify(
        &self,
        entities: Vec<i64>,
        remove_type: ERemoveEntityType,
    ) -> EntityRemoveNotify {
        EntityRemoveNotify {
            remove_infos: entities
                .iter()
                .map(|&entity_id| EntityRemoveInfo {
                    entity_id,
                    r#type: remove_type.into(),
                })
                .collect(),
            is_remove: true,
        }
    }

    pub fn build_update_group_formation_notify(
        &self,
        cur_formation: RoleFormation,
        world: &mut WorldEntity,
    ) -> UpdateGroupFormationNotify {
        let group_type = 1;
        UpdateGroupFormationNotify {
            group_formation: vec![GroupFormation {
                player_id: self.basic_info.id,
                fight_role_infos: vec![FightRoleInfos {
                    group_type,
                    fight_role_infos: cur_formation
                        .role_ids
                        .iter()
                        .map(|&role_id| FightRoleInfo {
                            role_id,
                            entity_id: world.get_entity_id(role_id),
                            on_stage_without_control: false,
                        })
                        .collect(),
                    cur_role: cur_formation.cur_role,
                    // is_retain: false,
                    living_status: LivingStatus::Alive.into(),
                    is_fixed_location: false,
                }],
                current_group_type: group_type,
            }],
        }
    }

    pub fn build_update_formation_notify(&self) -> UpdateFormationNotify {
        let role_map: HashMap<i32, &Role> = self
            .role_list
            .values()
            .map(|role| (role.role_id, role))
            .collect();

        UpdateFormationNotify {
            players_formations: vec![PlayerFightFormations {
                player_id: self.basic_info.id,
                formations: self
                    .formation_list
                    .iter()
                    .map(|(&formation_id, formation)| FightFormationNotifyInfo {
                        formation_id,
                        cur_role: formation.cur_role,
                        role_infos: formation
                            .role_ids
                            .iter()
                            .map(|role_id| {
                                if !role_map.contains_key(role_id) {
                                    tracing::warn!("Role {} not found in use role list", role_id);
                                    return Default::default();
                                }
                                role_map.get(role_id).unwrap().to_formation_protobuf()
                            })
                            .collect(),
                        is_current: formation.is_current,
                    })
                    .collect(),
            }],
        }
    }

    pub fn unlock_teleport(&mut self, teleport_id: i32) {
        let teleporter = wicked_waifus_data::teleporter_data::iter()
            .find(|teleporter| teleporter.id == teleport_id)
            .map(|teleporter| PlayerTeleport {
                id: teleporter.id,
                map_id: teleporter.map_id,
                entity_config_id: teleporter.teleport_entity_config_id,
            });

        if let Some(teleporter) = teleporter {
            self.teleports.teleports_data.push(teleporter);

            self.notify(HostTeleportUnlockNotify {
                host_player_id: self.basic_info.id,
                host_teleport_id: teleport_id,
            });

            self.notify(TeleportUpdateNotify {
                ids: vec![teleport_id],
            });
        }
    }

    pub fn unlock_tutorial(&mut self, tutorial_id: i32) -> PlayerTutorial {
        let tutorial = PlayerTutorial {
            id: tutorial_id,
            create_time: time_util::unix_timestamp() as u32,
            get_award: false,
        };
        self.tutorials.tutorials.push(tutorial.clone());
        tutorial
    }

    pub fn notify_month_card(&mut self) {
        if self.month_card.days > -1 && time_util::unix_days() != self.month_card.last_received_day
        {
            let astrites = self.inventory.add_astrite(90);
            self.month_card.days -= 1;
            self.month_card.last_received_day = time_util::unix_days();

            self.notify(PlayerAttrNotify {
                attributes: vec![PlayerAttr {
                    key: PlayerAttrKey::RareCoin.into(),
                    value_type: PlayerAttrType::Int32.into(),
                    value: Some(Value::Int32Value(astrites)),
                }],
            });
            self.notify(NormalItemUpdateNotify {
                normal_item_list: self.inventory.to_normal_item_list_filtered(&[3]),
                no_tips: false,
            });
            self.notify(MonthCardDailyRewardNotify {
                item_id: 3,
                count: 90,
                days: self.month_card.days,
            });
        }
    }

    pub fn load_from_save(save_data: PlayerSaveData) -> Self {
        let role_data = save_data.role_data.unwrap_or_default();

        Self {
            session: None,
            basic_info: PlayerBasicInfo::load_from_save(
                save_data.basic_data.clone().unwrap_or_default(),
            ),
            role_list: role_data
                .role_list
                .into_iter()
                .map(Role::load_from_save)
                .collect::<HashMap<i32, Role>>(),
            formation_list: role_data
                .role_formation_list
                .into_iter()
                .map(|(k, v)| (k, RoleFormation::load_from_save(v)))
                .collect(),
            cur_formation_id: role_data.cur_formation_id,
            location: save_data
                .location_data
                .map(PlayerLocation::load_from_save)
                .unwrap_or_default(),
            func: save_data
                .func_data
                .map(PlayerFunc::load_from_save)
                .unwrap_or_default(),
            explore_tools: save_data
                .explore_tools_data
                .map(ExploreTools::load_from_save)
                .unwrap_or_default(),
            player_chat: save_data
                .chat_data
                .map(PlayerChat::load_from_save)
                .unwrap_or_default(),
            guides: save_data
                .guides
                .map(PlayerGuides::load_from_save)
                .unwrap_or_default(),
            advise: save_data
                .advise
                .map(PlayerAdviceConfig::load_from_save)
                .unwrap_or_default(),
            adventure_status: save_data
                .adventure_status
                .map(PlayerAdventureStatus::load_from_save)
                .unwrap_or_default(),
            inventory: save_data
                .inventory
                .map(PlayerInventory::load_from_save)
                .unwrap_or_default(),
            teleports: save_data
                .teleports
                .map(PlayerTeleports::load_from_save)
                .unwrap_or_default(),
            tutorials: save_data
                .tutorials
                .map(PlayerTutorials::load_from_save)
                .unwrap_or_default(),
            map_trace: save_data
                .map_trace
                .map(PlayerMapTrace::load_from_save)
                .unwrap_or_default(),
            month_card: save_data
                .month_card
                .map(PlayerMonthCard::load_from_save)
                .unwrap_or_default(),
            mc_element: save_data
                .mc_element
                .map(PlayerMcElement::load_from_save)
                .unwrap_or_default(),
            unlocked_skins: save_data
                .unlocked_skins
                .map(PlayerUnlockedSkins::load_from_save)
                .unwrap_or_default(),
            world: Rc::new(RefCell::new(World::new())),
            last_save_time: time_util::unix_timestamp(),
            quadrant_id: 0,
        }
    }

    pub fn build_save_data(&self) -> PlayerSaveData {
        PlayerSaveData {
            basic_data: Some(self.basic_info.build_save_data()),
            role_data: Some(PlayerRoleData {
                role_list: self
                    .role_list
                    .iter()
                    .map(|(_, role)| role.build_save_data())
                    .collect(),
                role_formation_list: self
                    .formation_list
                    .iter()
                    .map(|(&k, v)| (k, v.build_save_data()))
                    .collect(),
                cur_formation_id: self.cur_formation_id,
            }),
            location_data: Some(self.location.build_save_data()),
            func_data: Some(self.func.build_save_data()),
            explore_tools_data: Some(self.explore_tools.build_save_data()),
            chat_data: Some(self.player_chat.build_save_data()),
            guides: Some(self.guides.build_save_data()),
            advise: Some(self.advise.build_save_data()),
            adventure_status: Some(self.adventure_status.build_save_data()),
            inventory: Some(self.inventory.build_save_data()),
            teleports: Some(self.teleports.build_save_data()),
            tutorials: Some(self.tutorials.build_save_data()),
            map_trace: Some(self.map_trace.build_save_data()),
            month_card: Some(self.month_card.build_save_data()),
            mc_element: Some(self.mc_element.build_save_data()),
            unlocked_skins: Some(self.unlocked_skins.build_save_data()),
        }
    }

    pub fn set_session(&mut self, session: Arc<Session>) {
        self.session = Some(session);
    }

    pub fn build_role_list_notify(&self) -> PbGetRoleListNotify {
        PbGetRoleListNotify {
            role_list: self
                .role_list
                .values()
                .map(|role| role.to_protobuf())
                .collect(),
        }
    }

    pub fn notify(&self, content: impl ProtocolUnit) {
        if let Some(session) = self.session.as_ref() {
            session.forward_to_gateway(Message::Push {
                sequence_number: 0,
                message_id: content.get_message_id(),
                payload: Some(content.encode_to_vec().into_boxed_slice()),
            });
        }
    }

    pub fn respond(&self, content: impl ProtocolUnit, rpc_id: u16) {
        if let Some(session) = self.session.as_ref() {
            let data = content.encode_to_vec().into_boxed_slice();
            let response = Message::Response {
                sequence_number: 0,
                message_id: content.get_message_id(),
                rpc_id,
                payload: Some(data),
            };
            session.forward_to_gateway(response);
        }
    }

    pub fn create_default_save_data(id: i32, name: String, sex: i32) -> PlayerSaveData {
        PlayerSaveData {
            basic_data: Some(PlayerBasicData {
                id,
                name,
                sex,
                level: 1,
                head_photo: 82001603,
                head_frame: 0,
                cur_map_id: 8,
                role_show_list: vec![Role::get_mc_id_based_on_sex(sex, Spectro)],
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
