use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct WeaponResonData {
    pub id: i32,
    pub reson_id: i32,
    pub level: i32,
    #[cfg(feature = "strict_json_fields")]
    pub name: String,
    #[cfg(feature = "strict_json_fields")]
    pub effect: Vec<i32>,
    pub consume: i32,
    pub gold_consume: i32,
    pub alternative_consume: Vec<i32>,
}