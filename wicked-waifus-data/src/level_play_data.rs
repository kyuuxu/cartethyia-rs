use serde::Deserialize;
use crate::LevelPlayDataDetail;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct LevelPlayDataData { // Json file contains Data in name, so it has to be DataData
    pub level_play_id: i32,
    pub data: LevelPlayDataDetail,
}

