use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GachaData {
    pub id: i32,
    pub rule_group_id: i32,
    pub sort: i32,
}