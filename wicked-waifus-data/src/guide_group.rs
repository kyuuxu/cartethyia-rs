use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GuideGroupData {
    pub id: i32,
    pub step: Vec<i32>,
    pub open_limit_condition: i32,
    pub auto_open_condition: i32,
    pub limit_repeat: Vec<i32>,
    pub dungeon_id: Vec<i32>,
    pub reset_in_dungeon: bool,
    pub online_mode: i32,
    pub priority: i32,
}