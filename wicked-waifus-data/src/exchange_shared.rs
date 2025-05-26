use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ExchangeSharedData {
    pub id: i32,
    pub max_count: i32,
    pub cost: HashMap<i32, i32>,
    pub reset_time_id: i32,
}