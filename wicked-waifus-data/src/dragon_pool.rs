use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DragonPoolData {
    pub id: i32,
    pub core_id: i32,
    pub goal: Vec<i32>,
    pub drop_ids: Vec<i32>,
    pub dark_coast_delivery_list: Vec<i32>,
    pub auto_take: bool,
}