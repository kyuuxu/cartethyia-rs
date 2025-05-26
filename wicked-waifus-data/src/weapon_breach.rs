use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct WeaponBreachData {
    pub id: i32,
    pub breach_id: i32,
    pub level: i32,
    pub condition_id: i32,
    pub level_limit: i32,
    pub consume: HashMap<i32, i32>,
    pub gold_consume: i32,
}
