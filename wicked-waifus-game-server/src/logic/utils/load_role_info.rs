use std::collections::HashMap;
use wicked_waifus_data::BasePropertyData;
use wicked_waifus_protocol::EAttributeType;

#[macro_export]
macro_rules! impl_base_prop {
    ($($name:ident),*) => {
        pub fn load_key_value(base_property: &BasePropertyData) -> HashMap<i32, i32> {
            HashMap::from([$(
                    ::paste::paste!((EAttributeType::[<$name:camel>] as i32, base_property.$name as i32)),
                )*])
        }
        pub fn attribute_from_data(base_property: &BasePropertyData, add_property: Option<&BasePropertyData>) -> HashMap<EAttributeType, (i32, i32)> {
            HashMap::from([$(
                    ::paste::paste!((EAttributeType::[<$name:camel>], (base_property.$name, add_property.map_or(0, |v| v.$name)))),
                )*])
        }
        pub fn from_key_value(key_value: &HashMap<i32, i32>) -> BasePropertyData {
            let mut result = BasePropertyData::default();
            $(::paste::paste! {
                    if let Some(value) = key_value.get(&(EAttributeType::[<$name:camel>] as i32)) {
                        result.$name = *value;
                    }
                }
            )*
            result
        }
    };
}

impl_base_prop!(
    lv,
    life_max,
    life,
    sheild,
    sheild_damage_change,
    sheild_damage_reduce,
    atk,
    crit,
    crit_damage,
    def,
    energy_efficiency,
    cd_reduse,
    element_efficiency,
    damage_change_normal_skill,
    damage_change,
    damage_reduce,
    damage_change_auto,
    damage_change_cast,
    damage_change_ultra,
    damage_change_qte,
    damage_change_phys,
    damage_change_element1,
    damage_change_element2,
    damage_change_element3,
    damage_change_element4,
    damage_change_element5,
    damage_change_element6,
    damage_resistance_phys,
    damage_resistance_element1,
    damage_resistance_element2,
    damage_resistance_element3,
    damage_resistance_element4,
    damage_resistance_element5,
    damage_resistance_element6,
    heal_change,
    healed_change,
    damage_reduce_phys,
    damage_reduce_element1,
    damage_reduce_element2,
    damage_reduce_element3,
    damage_reduce_element4,
    damage_reduce_element5,
    damage_reduce_element6,
    energy_max,
    energy,
    special_energy_1_max,
    special_energy_1,
    special_energy_2_max,
    special_energy_2,
    special_energy_3_max,
    special_energy_3,
    special_energy_4_max,
    special_energy_4,
    strength_max,
    strength,
    strength_recover,
    strength_punish_time,
    strength_run,
    strength_swim,
    strength_fast_swim,
    hardness_max,
    hardness,
    hardness_recover,
    hardness_punish_time,
    hardness_change,
    hardness_reduce,
    rage_max,
    rage,
    rage_recover,
    rage_punish_time,
    rage_change,
    rage_reduce,
    tough_max,
    tough,
    tough_recover,
    tough_change,
    tough_reduce,
    tough_recover_delay_time,
    element_power1,
    element_power2,
    element_power3,
    element_power4,
    element_power5,
    element_power6,
    special_damage_change,
    strength_fast_climb_cost,
    element_property_type,
    weak_time,
    ignore_def_rate,
    ignore_damage_resistance_phys,
    ignore_damage_resistance_element1,
    ignore_damage_resistance_element2,
    ignore_damage_resistance_element3,
    ignore_damage_resistance_element4,
    ignore_damage_resistance_element5,
    ignore_damage_resistance_element6,
    skill_tough_ratio,
    strength_climb_jump,
    strength_gliding,
    mass,
    braking_friction_factor,
    gravity_scale,
    speed_ratio,
    damage_change_phantom,
    auto_attack_speed,
    cast_attack_speed,
    status_build_up_1_max,
    status_build_up_1,
    status_build_up_2_max,
    status_build_up_2,
    status_build_up_3_max,
    status_build_up_3,
    status_build_up_4_max,
    status_build_up_4,
    status_build_up_5_max,
    status_build_up_5,
    paralysis_time_max,
    paralysis_time,
    paralysis_time_recover,
    element_energy_max,
    element_energy
);
