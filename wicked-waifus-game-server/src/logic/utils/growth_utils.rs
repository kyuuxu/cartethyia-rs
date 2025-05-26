use wicked_waifus_data::{BasePropertyData, base_property_data, monster_property_growth_data, role_property_growth_data};

pub fn get_role_props_by_level(id: i32, level: i32, breach: i32) -> BasePropertyData {
    let mut base_props = get_role_props_or_default(id).clone();
    let role_props_growth = role_property_growth_data::iter()
        .find(|d| d.level == level && d.breach_level == breach)
        .unwrap_or_else(|| {
            role_property_growth_data::iter().find(|d| d.level == 1 && d.breach_level == 0).unwrap()
        });
    base_props.lv = level;
    compute_scaled_stat(&mut base_props.life_max, role_props_growth.life_max_ratio);
    compute_scaled_stat(&mut base_props.life, role_props_growth.life_max_ratio);
    compute_scaled_stat(&mut base_props.atk, role_props_growth.atk_ratio);
    compute_scaled_stat(&mut base_props.def, role_props_growth.def_ratio);
    base_props
}

pub fn get_monster_props_by_level(id: i32, level: i32) -> BasePropertyData {
    let mut base_props = base_property_data::iter()
        .find(|d| d.id == id)
        .unwrap() // TODO: Default??
        .clone();
    // TODO: Check if curve_id matters
    let monster_props_growth = monster_property_growth_data::iter()
        .find(|d| d.level == level)
        .unwrap(); // TODO: Default??
    // Compute scaled properties
    base_props.lv = level;
    compute_scaled_stat(&mut base_props.life_max, monster_props_growth.life_max_ratio);
    compute_scaled_stat(&mut base_props.life, monster_props_growth.life_max_ratio);
    compute_scaled_stat(&mut base_props.atk, monster_props_growth.atk_ratio);
    compute_scaled_stat(&mut base_props.def, monster_props_growth.def_ratio);
    compute_scaled_stat(&mut base_props.hardness_max, monster_props_growth.hardness_max_ratio);
    compute_scaled_stat(&mut base_props.hardness, monster_props_growth.hardness_ratio);
    compute_scaled_stat(&mut base_props.hardness_recover, monster_props_growth.hardness_recover_ratio);
    compute_scaled_stat(&mut base_props.rage_max, monster_props_growth.rage_max_ratio);
    compute_scaled_stat(&mut base_props.rage, monster_props_growth.rage_ratio);
    compute_scaled_stat(&mut base_props.rage_recover, monster_props_growth.rage_recover_ratio);
    base_props
}

#[inline(always)]
fn get_role_props_or_default(id: i32) -> &'static BasePropertyData {
    base_property_data::iter()
        .find(|d| d.id == id)
        .unwrap_or_else(|| {
            base_property_data::iter().find(|d| d.id == 1102).unwrap()
        })
}

#[inline(always)]
fn compute_scaled_stat(stat: &mut i32, ratio: i32) {
    *stat = (*stat as f32 * (ratio as f32 / 10000f32)).trunc() as i32;
}