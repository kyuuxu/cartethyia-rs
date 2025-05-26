use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct WeaponPropertyGrowthData {
    pub id: i32,
    pub curve_id: i32,
    pub level: i32,
    pub breach_level: i32,
    pub curve_value: i32,
}