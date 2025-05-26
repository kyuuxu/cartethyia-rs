use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct MonsterPropertyGrowthData {
    pub id: i32,
    pub curve_id: i32,
    pub level: i32,
    pub life_max_ratio: i32,
    pub atk_ratio: i32,
    pub def_ratio: i32,
    pub hardness_max_ratio: i32,
    pub hardness_ratio: i32,
    pub hardness_recover_ratio: i32,
    pub rage_max_ratio: i32,
    pub rage_ratio: i32,
    pub rage_recover_ratio: i32,
}