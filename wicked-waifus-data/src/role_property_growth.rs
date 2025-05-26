use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RolePropertyGrowthData {
    pub id: i32,
    pub level: i32,
    pub breach_level: i32,
    pub life_max_ratio: i32,
    pub atk_ratio: i32,
    pub def_ratio: i32,
}