use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RoleInfoData {
    pub id: i32,
    pub quality_id: i32,
    pub role_type: i32,
    pub is_trial: bool,
    #[cfg(feature = "strict_json_fields")]
    pub name: String,
    #[cfg(feature = "strict_json_fields")]
    pub nick_name: String,
    #[cfg(feature = "strict_json_fields")]
    pub introduction: String,
    pub tag: Vec<i32>,
    pub parent_id: i32,
    pub priority: i32,
    pub show_property: Vec<i32>,
    pub element_id: i32,
    pub skin_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub role_head_icon_circle: String,
    #[cfg(feature = "strict_json_fields")]
    pub role_head_icon_large: String,
    #[cfg(feature = "strict_json_fields")]
    pub role_head_icon_big: String,
    #[cfg(feature = "strict_json_fields")]
    pub card: String,
    #[cfg(feature = "strict_json_fields")]
    pub role_head_icon: String,
    #[cfg(feature = "strict_json_fields")]
    pub formation_role_card: String,
    #[cfg(feature = "strict_json_fields")]
    pub role_stand: String,
    #[cfg(feature = "strict_json_fields")]
    pub role_portrait: String,
    pub spillover_item: HashMap<i32, i32>,
    #[cfg(feature = "strict_json_fields")]
    pub mesh_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub ui_mesh_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub role_body: String,
    pub breach_model: i32,
    pub special_energy_bar_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub camera_config: String,
    #[cfg(feature = "strict_json_fields")]
    pub camera_float_height: i32,
    pub entity_property: i32,
    pub max_level: i32,
    pub level_consume_id: i32,
    pub breach_id: i32,
    pub skill_id: i32,
    pub skill_tree_group_id: i32,
    pub resonance_id: i32,
    pub resonant_chain_group_id: i32,
    pub is_show: bool,
    pub exchange_consume: HashMap<i32, i32>,
    pub init_weapon_item_id: i32,
    pub weapon_type: i32,
    #[cfg(feature = "strict_json_fields")]
    #[serde(rename = "SkillDAPath")]
    pub skill_da_path: String,
    #[cfg(feature = "strict_json_fields")]
    #[serde(rename = "SkillLockDAPath")]
    pub skill_lock_da_path: String,
    #[cfg(feature = "strict_json_fields")]
    #[serde(rename = "UiScenePerformanceABP")]
    pub ui_scene_performance_abp: String,
    pub lock_on_default_id: i32,
    pub lock_on_look_on_id: i32,
    #[cfg(feature = "strict_json_fields")]
    #[serde(rename = "SkillEffectDA")]
    pub skill_effect_da: String,
    #[cfg(feature = "strict_json_fields")]
    pub foot_step_state: String,
    pub party_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub attributes_description: String,
    #[cfg(feature = "strict_json_fields")]
    pub icon: String,
    pub item_quality_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub obtained_show_description: String,
    pub num_limit: i32,
    pub show_in_bag: bool,
    #[cfg(feature = "strict_json_fields")]
    pub weapon_scale: Vec<i32>,
    #[cfg(feature = "strict_json_fields")]
    pub intervene: bool,
    #[cfg(feature = "strict_json_fields")]
    pub character_voice: String,
    pub trial_role: i32,
    pub is_aim: bool,
    pub role_guide: i32,
    #[cfg(feature = "strict_json_fields")]
    pub red_dot_disable_rule: i32,
    #[cfg(feature = "strict_json_fields")]
    pub skin_damage: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub hide_hu_lu: bool,
}
