use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct WeaponExpItemData {
    pub id: i32,
    pub cost: i32,
    pub basic_exp: i32,
}
