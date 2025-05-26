use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AchievementData {
    pub id: i32,
    pub group_id: i32,
    pub level: i32,
    #[cfg(feature = "strict_json_fields")]
    pub name: String,
    #[cfg(feature = "strict_json_fields")]
    pub desc: String,
    #[cfg(feature = "strict_json_fields")]
    pub icon_path: String,
    pub override_drop_id: i32,
    pub hidden: bool,
    pub next_link: i32,
    pub client_trigger: bool,
    pub third_party_trophy_id: i32,
}