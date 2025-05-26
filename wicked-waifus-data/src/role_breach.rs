use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RoleBreachData {
    pub id: i32,
    pub breach_group_id: i32,
    pub breach_level: i32,
    pub max_level: i32,
    pub breach_consume: HashMap<i32, i32>,
    pub breach_reward: i32,
    pub condition_id: i32,
}
