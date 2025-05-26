#[cfg(feature = "strict_json_fields")]
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AdventureTaskData {
    pub id: i32,
    pub chapter_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub task_text: String,
    pub record_id: Vec<i32>,
    pub need_progress: i32,
    pub drop_ids: i32,
    pub path_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub jump_to: HashMap<String, String>,
}