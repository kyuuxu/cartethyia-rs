use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AreaData {
    pub area_id: i32,
    pub level: i32,
    pub country_id: i32,
    pub delivery_mark_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub area_name: String,
    pub map_config_id: i32,
    pub dungeon_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub title: String,
    pub father: i32,
    pub tag: Vec<i32>,
    pub record: i32,
    pub tips: i32,
    pub is_init_actived: bool,
    #[serde(rename = "WorldMonsterLevelMax")]
    pub world_monster_level_max: HashMap<i32, i32>,
    #[serde(rename = "WuYinQuID")]
    pub wu_yin_qu_id: i32,
    pub state_id: i32,
    pub atmosphere_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub edge_wall_name: String,
    pub delivery_mark_type: i32,
    pub sort_index: i32,
    pub enter_area_tags: HashMap<i32, i32>,
    pub leave_area_tags: HashMap<i32, i32>,
}