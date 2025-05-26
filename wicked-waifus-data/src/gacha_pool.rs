use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GachaPoolData {
    pub id: i32,
    pub gacha_id: i32,
    pub sort: i32,
}