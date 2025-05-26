use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RoleLevelConsumeData {
    pub id: i32,
    pub consume_group_id: i32,
    pub level: i32,
    pub exp_count: i32,
}
