use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct WeaponLevelData {
    pub id: i32,
    pub level_id: i32,
    pub level: i32,
    pub exp: i32,
}
