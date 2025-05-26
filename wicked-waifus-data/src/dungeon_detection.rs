use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DungeonDetectionData {
    pub id: i32,
    pub dungeon_id: i32,
    pub name: String,
    pub role_id: i32,
    pub guide_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub level_play_list: Vec<Option<serde_json::Value>>,
    #[cfg(feature = "strict_json_fields")]
    pub instance_sub_type_description: String,
    pub type_description1: i32,
    pub secondary: i32,
    pub mat_type: i32,
    #[cfg(feature = "strict_json_fields")]
    pub type_description2: String,
    #[cfg(feature = "strict_json_fields")]
    pub attributes_description_lock: String,
    #[cfg(feature = "strict_json_fields")]
    pub attributes_description_unlock: String,
    #[cfg(feature = "strict_json_fields")]
    pub big_icon: String,
    #[cfg(feature = "strict_json_fields")]
    pub icon: String,
    #[cfg(feature = "strict_json_fields")]
    pub lock_big_icon: String,
    #[cfg(feature = "strict_json_fields")]
    pub temporary_icon_un_lock: String,
    #[cfg(feature = "strict_json_fields")]
    pub temporary_iconlock: String,
    pub show_reward: i32,
    pub show_reward_map: HashMap<String, i32>,
    pub begin_time_stamp: i32,
    pub pre_open_id: i32,
    pub lock_con: i32,
    #[cfg(feature = "strict_json_fields")]
    pub phantom_id: Vec<Option<serde_json::Value>>,
    pub sub_dungeon_id: i32,
    pub sort_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub new_content: String,
}