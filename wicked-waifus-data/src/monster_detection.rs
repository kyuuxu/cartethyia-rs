use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct MonsterDetectionData {
    pub id: i32,
    pub blueprint_type: String,
    pub mark_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub name: String,
    pub show_reward: i32,
    pub entity_config_id: i32,
    pub danger_type: i32,
    pub type_description2: i32,
    #[cfg(feature = "strict_json_fields")]
    pub attributes_description_lock: String,
    #[cfg(feature = "strict_json_fields")]
    pub attributes_description_unlock: String,
    #[cfg(feature = "strict_json_fields")]
    pub big_icon: String,
    #[cfg(feature = "strict_json_fields")]
    pub icon: String,
    #[cfg(feature = "strict_json_fields")]
    pub temporary_icon_un_lock: String,
    #[cfg(feature = "strict_json_fields")]
    pub temporary_iconlock: String,
    pub begin_time_stamp: i32,
    pub lock_con: i32,
    pub monster_info_id: i32,
}