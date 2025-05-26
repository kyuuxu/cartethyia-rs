use std::{collections::HashMap, hash::Hash};

use wicked_waifus_protocol::{ArrayIntInt, FormationRoleInfo, RoleInfo};

use crate::config;
use crate::logic::player::Element;
use crate::logic::utils::growth_utils::get_role_props_by_level;
use crate::logic::utils::load_role_info::load_key_value;
pub use formation::RoleFormation;
use wicked_waifus_commons::time_util;
use wicked_waifus_data::{role_info_data, BasePropertyData};
use wicked_waifus_protocol_internal::{PlayerInventoryWeaponData, RoleData, RoleStats};

use super::{math::Weapon, utils::growth_utils::get_weapon_props};

const MAIN_CHARACTER_MALE_SPECTRO_ID: i32 = 1501;
const MAIN_CHARACTER_FEMALE_SPECTRO_ID: i32 = 1502;
const MAIN_CHARACTER_MALE_HAVOC_ID: i32 = 1605;
const MAIN_CHARACTER_FEMALE_HAVOC_ID: i32 = 1604;
const MAIN_CHARACTER_MALE_AERO_ID: i32 = 1406;
const MAIN_CHARACTER_FEMALE_AERO_ID: i32 = 1408;

const MAIN_CHARACTER_ARRAY: &[i32] = &[
    MAIN_CHARACTER_MALE_SPECTRO_ID,
    MAIN_CHARACTER_FEMALE_SPECTRO_ID,
    MAIN_CHARACTER_MALE_HAVOC_ID,
    MAIN_CHARACTER_FEMALE_HAVOC_ID,
    MAIN_CHARACTER_MALE_AERO_ID,
    MAIN_CHARACTER_FEMALE_AERO_ID,
];

mod formation;
pub struct Role {
    pub role_id: i32,
    pub name: String,
    pub level: i32,
    pub exp: i32,
    pub breakthrough: i32,
    pub skill_map: HashMap<i32, i32>,
    pub star: i32,
    pub favor: i32,
    pub create_time: u32,
    pub equip_weapon: i32,
    pub skin_id: i32,
    pub resonant_chain_group_index: i32,
    pub base_prop: HashMap<i32, i32>,
    pub add_prop: HashMap<i32, i32>,
    pub hp: i32,
    pub energy: i32,
    pub special_energy_1: i32,
    pub special_energy_2: i32,
    pub special_energy_3: i32,
    pub special_energy_4: i32,
    pub element_energy: i32,
    pub favor_level: i32,
    pub favor_exp: i32,
    pub wing_skin_id: i32,
    pub fly_skin_id: i32,
    pub weapon_skin_id: i32,
}

impl Role {
    #[inline(always)]
    pub fn get_all_roles_except_mc() -> Vec<i32> {
        role_info_data::iter()
            .filter(|role| {
                (role.role_type == 1 && !MAIN_CHARACTER_ARRAY.contains(&role.id)) || role.id == 5101
            })
            .map(|role| role.id)
            .collect()
    }

    #[inline(always)]
    pub fn get_mc_id_based_on_sex(sex: i32, current_element: Element) -> i32 {
        match current_element {
            Element::Glacio => unreachable!(),
            Element::Fusion => unreachable!(),
            Element::Electro => unreachable!(),
            Element::Aero => match sex {
                0 => MAIN_CHARACTER_MALE_AERO_ID,
                1 => MAIN_CHARACTER_FEMALE_AERO_ID,
                _ => unreachable!(),
            },
            Element::Spectro => match sex {
                0 => MAIN_CHARACTER_FEMALE_SPECTRO_ID,
                1 => MAIN_CHARACTER_MALE_SPECTRO_ID,
                _ => unreachable!(),
            },
            Element::Havoc => match sex {
                0 => MAIN_CHARACTER_MALE_HAVOC_ID,
                1 => MAIN_CHARACTER_FEMALE_HAVOC_ID,
                _ => unreachable!(),
            },
        }
    }

    #[inline(always)]
    pub fn get_mc_based_on_sex(sex: i32, current_element: Element) -> Self {
        Role::new(Self::get_mc_id_based_on_sex(sex, current_element))
    }

    pub fn get_mc_unlock_variations(unlocked_elements: Vec<Element>) -> Vec<i32> {
        let mut variations: Vec<i32> = Vec::new();
        for element in unlocked_elements {
            match element {
                Element::Glacio => {}
                Element::Fusion => {}
                Element::Electro => {}
                Element::Aero => {
                    variations.push(MAIN_CHARACTER_MALE_AERO_ID);
                    variations.push(MAIN_CHARACTER_FEMALE_AERO_ID);
                }
                Element::Spectro => {
                    variations.push(MAIN_CHARACTER_MALE_SPECTRO_ID);
                    variations.push(MAIN_CHARACTER_FEMALE_SPECTRO_ID);
                }
                Element::Havoc => {
                    variations.push(MAIN_CHARACTER_MALE_HAVOC_ID);
                    variations.push(MAIN_CHARACTER_FEMALE_HAVOC_ID);
                }
            }
        }
        variations
    }

    pub fn new(role_id: i32) -> Self {
        let data = role_info_data::iter().find(|d| d.id == role_id).unwrap();

        let default_unlocks = &config::get_config().default_unlocks;
        let (level, breakthrough) = if default_unlocks.unlock_all_roles_max_level {
            (
                data.max_level,
                wicked_waifus_data::role_breach_data::iter()
                    .filter(|level_data| level_data.breach_group_id == data.breach_id)
                    .map(|level_data| level_data.breach_level)
                    .max()
                    .unwrap_or(0),
            )
        } else {
            (
                1,
                wicked_waifus_data::role_breach_data::iter()
                    .filter(|level_data| level_data.breach_group_id == data.breach_id)
                    .map(|level_data| level_data.breach_level)
                    .min()
                    .unwrap_or(0),
            )
        };
        let resonant_chain_group_index = if default_unlocks.unlock_all_roles_all_sequences {
            wicked_waifus_data::resonant_chain_data::iter()
                .filter(|level_data| level_data.group_id == data.resonant_chain_group_id)
                .map(|level_data| level_data.group_index)
                .max()
                .unwrap_or(0)
        } else {
            wicked_waifus_data::resonant_chain_data::iter()
                .filter(|level_data| level_data.group_id == data.resonant_chain_group_id)
                .map(|level_data| level_data.group_index)
                .min()
                .unwrap_or(0)
        };
        // TODO: add weapon and echo stats
        let base_stats = &get_role_props_by_level(role_id, level, breakthrough);
        let base_prop = load_key_value(base_stats);
        let add_prop: HashMap<i32, i32> = HashMap::from([
            (2, 0),  // HP
            (7, 0),  // ATK
            (10, 0), // DEF
        ]);

        Self {
            role_id,
            name: String::with_capacity(0),
            level,
            exp: 0,
            breakthrough,
            skill_map: HashMap::new(), // TODO!
            star: 0,
            favor: 0,
            create_time: time_util::unix_timestamp() as u32,
            equip_weapon: data.init_weapon_item_id,
            skin_id: data.skin_id,
            resonant_chain_group_index,
            base_prop,
            add_prop,
            hp: base_stats.life,
            energy: base_stats.energy,
            special_energy_1: base_stats.special_energy_1,
            special_energy_2: base_stats.special_energy_2,
            special_energy_3: base_stats.special_energy_3,
            special_energy_4: base_stats.special_energy_4,
            element_energy: base_stats.element_energy,
            favor_level: 0,
            favor_exp: 0,
            wing_skin_id: 0,
            fly_skin_id: 0,
            weapon_skin_id: 0,
        }
    }

    pub fn get_base_properties(&self) -> BasePropertyData {
        // Overwrite dynamic attributes with stores values
        let mut base_stats = get_role_props_by_level(self.role_id, self.level, self.breakthrough);
        // TODO: add weapon and echo stats
        // TODO: Integrity check, value has to be between 0 and max
        base_stats.life = self.hp;
        base_stats.energy = self.energy;
        base_stats.special_energy_1 = self.special_energy_1;
        base_stats.special_energy_2 = self.special_energy_2;
        base_stats.special_energy_3 = self.special_energy_3;
        base_stats.special_energy_4 = self.special_energy_4;
        base_stats.element_energy = self.element_energy;
        base_stats
    }

    pub fn compute_role_equipment_effects(&mut self, weapon: &PlayerInventoryWeaponData) {
        // Apply Weapon Atributes
        Weapon::assign_weapon_attributes(&mut self.base_prop, &mut self.add_prop, weapon);

        // TODO: Apply Echoes Atributes
    }

    pub fn to_protobuf(&self) -> RoleInfo {
        RoleInfo {
            role_id: self.role_id,
            name: self.name.clone(),
            level: self.level,
            exp: self.exp,
            breakthrough: self.breakthrough,
            create_time: self.create_time,
            skills: self
                .skill_map
                .iter()
                .map(|(k, v)| ArrayIntInt { key: *k, value: *v })
                .collect(),
            base_prop: self
                .base_prop
                .iter()
                .map(|(&k, &v)| ArrayIntInt { key: k, value: v })
                .collect(),
            add_prop: self
                .add_prop
                .iter()
                .map(|(&k, &v)| ArrayIntInt { key: k, value: v })
                .collect(),
            skin_id: self.skin_id,
            resonant_chain_group_index: self.resonant_chain_group_index,
            ..Default::default()
        }
    }

    pub fn to_formation_protobuf(&self) -> FormationRoleInfo {
        let base_stats = get_role_props_by_level(self.role_id, self.level, self.breakthrough);
        FormationRoleInfo {
            role_id: self.role_id,
            max_hp: base_stats.life_max,
            cur_hp: base_stats.life,
            level: self.level,
            role_skin_id: self.skin_id,
        }
    }

    pub fn load_from_save(data: RoleData) -> (i32, Self) {
        (
            data.role_id,
            Self {
                role_id: data.role_id,
                name: data.name,
                level: data.level,
                exp: data.exp,
                breakthrough: data.breakthrough,
                skill_map: data.skill_map,
                star: data.star,
                favor: data.favor,
                create_time: data.create_time,
                equip_weapon: data.equip_weapon,
                skin_id: data.skin_id,
                resonant_chain_group_index: data.resonant_chain_group_index,
                base_prop: if data.base_prop.is_empty() {
                    load_key_value(&get_role_props_by_level(
                        data.role_id,
                        data.level,
                        data.breakthrough,
                    ))
                } else {
                    data.base_prop
                },
                add_prop: data.add_prop,
                hp: data.stats.unwrap().hp,
                energy: data.stats.unwrap().energy,
                special_energy_1: data.stats.unwrap().special_energy_1,
                special_energy_2: data.stats.unwrap().special_energy_2,
                special_energy_3: data.stats.unwrap().special_energy_3,
                special_energy_4: data.stats.unwrap().special_energy_4,
                element_energy: data.stats.unwrap().element_energy,
                favor_level: data.favor_level,
                favor_exp: data.favor_exp,
                wing_skin_id: data.wing_skin_id,
                fly_skin_id: data.fly_skin_id,
                weapon_skin_id: data.weapon_skin_id,
            },
        )
    }

    pub fn build_save_data(&self) -> RoleData {
        RoleData {
            role_id: self.role_id,
            name: self.name.clone(),
            level: self.level,
            exp: self.exp,
            breakthrough: self.breakthrough,
            skill_map: self.skill_map.clone(),
            // phantom_map: Default::default(),
            star: self.star,
            favor: self.favor,
            create_time: self.create_time,
            // cur_model: 0,
            // models: vec![],
            // skill_node_state: vec![],
            resonant_chain_group_index: self.resonant_chain_group_index,
            base_prop: HashMap::new(), // Naruse suggest me to don't save baseprop and addprop to db
            add_prop: HashMap::new(),
            equip_weapon: self.equip_weapon,
            skin_id: self.skin_id,
            stats: Some(RoleStats {
                hp: self.hp,
                energy: self.energy,
                special_energy_1: self.special_energy_1,
                special_energy_2: self.special_energy_2,
                special_energy_3: self.special_energy_3,
                special_energy_4: self.special_energy_4,
                element_energy: self.element_energy,
            }),
            favor_level: self.favor_level,
            favor_exp: self.favor_exp,
            wing_skin_id: self.wing_skin_id,
            fly_skin_id: self.fly_skin_id,
            weapon_skin_id: self.weapon_skin_id,
            ..Default::default()
        }
    }
}
