use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct LivenessTaskData {
    pub task_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub task_name: String,
    pub update_type: i32,
    pub task_reward: HashMap<i32, i32>,
    #[cfg(feature = "strict_json_fields")]
    pub task_func: Vec<String>,
    pub unlock_condition: i32,
    pub sort_rank: i32,
}