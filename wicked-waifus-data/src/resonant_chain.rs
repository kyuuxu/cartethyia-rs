use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ResonantChainData {
    pub id: i32,
    pub group_id: i32,
    pub group_index: i32,
    pub node_type: i32,
    #[cfg(feature = "strict_json_fields")]
    pub node_index: String,
    #[cfg(feature = "strict_json_fields")]
    pub node_name: String,
    #[cfg(feature = "strict_json_fields")]
    pub attributes_description: String,
    #[cfg(feature = "strict_json_fields")]
    pub bg_description: String,
    pub buff_ids: Vec<i64>,
    pub add_prop: Vec<i32>,
    pub activate_consume: HashMap<i32, i32>,
    #[cfg(feature = "strict_json_fields")]
    pub attributes_description_params: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub node_icon: String,
}
