use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ItemExchangeContentData {
    pub item_id: i32,
    pub times: i32,
    pub gain_count: i32,
    pub consume: HashMap<i32, i32>,
}