use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ExploreProgressData {
    pub id: i32,
    pub area: i32,
    pub explore_type: i32,
    pub sub_type_score: HashMap<i32, i32>,
    pub phantom_skill_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub unlock_text_id: String,
    #[cfg(feature = "strict_json_fields")]
    pub lock_text_id: String,
    pub unlock_condition: i32,
    #[cfg(feature = "strict_json_fields")]
    pub special_player_map: HashMap<i32, i32>,
    pub is_recommend: bool,
    pub is_show_progress: bool,
    pub is_show_track: bool,
    #[cfg(feature = "strict_json_fields")]
    pub special_player_desc: String,
}