use std::collections::HashMap;

use wicked_waifus_protocol_internal::PlayerInventoryWeaponData;

use crate::logic::utils::growth_utils::get_weapon_props;

pub struct Weapon;
pub struct Echoes;
pub struct Buff;
pub struct Damage;

impl Weapon {
    pub fn assign_weapon_attributes(
        base_prop: &mut HashMap<i32, i32>,
        add_prop: &mut HashMap<i32, i32>,
        weapon: &PlayerInventoryWeaponData,
    ) -> HashMap<i32, (i32, bool)> {
        let weapon_attributes = get_weapon_props(&weapon);
        let mut result: HashMap<i32, (i32, bool)> = HashMap::new();

        for (is_ratio, key, value) in weapon_attributes {
            let normalize_key = key - 10000;

            if is_ratio && key > 1000 {
                if let Some(attr_ref) = base_prop.get_mut(&normalize_key) {
                    let before = *attr_ref;
                    let computed = (before as f32 * (value as f32 / 1e4)) as i32;

                    let entry = add_prop.entry(normalize_key).or_insert(0);
                    *entry += computed;

                    result.insert(normalize_key, (*entry, is_ratio));
                }
            } else {
                let entry = base_prop.entry(key).or_insert(0);
                *entry += value;

                result.insert(key, (*entry, is_ratio));
            }
        }

        result
    }

    pub fn remove_weapon_attributes(
        base_prop: &mut HashMap<i32, i32>,
        add_prop: &mut HashMap<i32, i32>,
        weapon: &PlayerInventoryWeaponData,
    ) -> HashMap<i32, (i32, bool)> {
        let weapon_attributes = get_weapon_props(&weapon);
        let mut result: HashMap<i32, (i32, bool)> = HashMap::new();

        for (is_ratio, key, value) in weapon_attributes {
            let normalize_key = key - 10000;

            if is_ratio && key > 1000 {
                if let Some(attr_ref) = base_prop.get_mut(&normalize_key) {
                    let before = *attr_ref;
                    let computed = (before as f32 * (value as f32 / 1e4)) as i32;

                    let entry = add_prop.entry(normalize_key).or_insert(0);
                    *entry -= computed;

                    result.insert(normalize_key, (*entry, is_ratio));
                }
            } else {
                let entry = base_prop.entry(key).or_insert(0);
                *entry -= value;

                result.insert(key, (*entry, is_ratio));
            }
        }

        result
    }
}
