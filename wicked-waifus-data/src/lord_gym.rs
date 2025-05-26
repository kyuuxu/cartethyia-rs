use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct LordGymData {
    pub id: i32,
    pub difficulty: i32,
    pub reward_id: i32,
    pub play_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub gym_title: String,
    #[cfg(feature = "strict_json_fields")]
    pub new_gym_title: String,
    #[cfg(feature = "strict_json_fields")]
    pub icon_path: String,
    #[cfg(feature = "strict_json_fields")]
    pub play_description: String,
    pub help_id: i32,
    pub monster_list: Vec<i32>,
    pub monster_level: i32,
    pub lock_con: i32,
    #[cfg(feature = "strict_json_fields")]
    pub lock_description: String,
    pub filter_type: i32,
    pub is_new: bool,
    pub is_debug: bool,
}