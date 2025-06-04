use std::collections::HashMap;

use wicked_waifus_data::damage_data;
use wicked_waifus_protocol::EAttributeType;
use wicked_waifus_protocol_internal::PlayerInventoryWeaponData;

use crate::logic::utils::growth_utils::get_weapon_props;
use tracing::debug;

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

impl Damage {
    pub fn attribute_recovery(
        damage_id: i64,
        base_prop: &mut HashMap<i32, i32>,
    ) -> HashMap<i32, i32> {
        let mut attributes: HashMap<i32, i32> = HashMap::new();

        // ENERGY REGEN
        {
            let energy_regen = damage_data::iter()
                .find(|d| d.id == damage_id)
                .and_then(|data| {
                    let base_energy = data.energy.get(0).unwrap_or(&0);
                    let energy_efficiency = base_prop
                        .get(&EAttributeType::EnergyEfficiency.into())
                        .unwrap_or(&0);

                    let recovered_energy =
                        (*base_energy as f32 * (*energy_efficiency as f32 / 1e4)) as i32;

                    Some(recovered_energy)
                })
                .unwrap_or(0);

            let energy_max_key = EAttributeType::EnergyMax.into();
            let energy_key = EAttributeType::Energy.into();

            if let Some(attr_max_val) = base_prop.get(&energy_max_key).copied() {
                if let Some(attr) = base_prop.get_mut(&energy_key) {
                    *attr = (*attr + energy_regen).clamp(0, attr_max_val);
                    attributes.insert(energy_key, *attr);
                }
            }
        }

        // CONCERTO REGEN
        {
            let concerto_regen = damage_data::iter()
                .find(|d| d.id == damage_id)
                .and_then(|data| {
                    let base_concerto = data.element_power.get(0).unwrap_or(&0);
                    let concerto_efficiency = base_prop
                        .get(&EAttributeType::ElementEfficiency.into())
                        .unwrap_or(&0);

                    let recovered_concerto =
                        (*base_concerto as f32 * (*concerto_efficiency as f32 / 1e4)) as i32;

                    Some(recovered_concerto)
                })
                .unwrap_or(0);

            let concerto_max_key = EAttributeType::ElementEnergyMax.into();
            let concerto_key = EAttributeType::ElementEnergy.into(); 

            if let Some(attr_max_val) = base_prop.get(&concerto_max_key).copied() {
                if let Some(attr) = base_prop.get_mut(&concerto_key) {
                    *attr = (*attr + concerto_regen).clamp(0, attr_max_val);
                    attributes.insert(concerto_key, *attr);
                }
            }
        }

        attributes
    }
}
