use serde::Deserialize;
use crate::node_data::NodeDataDetail;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct LevelPlayNodeDataData { // Json file contains Data in name, so it has to be DataData
    pub key: String,
    pub data: NodeDataDetail,
}

