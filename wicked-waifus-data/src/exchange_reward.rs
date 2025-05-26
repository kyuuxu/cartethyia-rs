use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ExchangeRewardData {
    pub id: i32,
    pub shared_id: i32,
    pub max_count: i32,
    pub cost: HashMap<i32, i32>,
    pub reward_id: HashMap<i32, i32>,
    pub preview_reward: HashMap<i32, i32>,
}