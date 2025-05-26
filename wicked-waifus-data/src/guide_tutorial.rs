use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GuideTutorialData {
    pub id: i32,
    pub tutorial_type: i32,
    pub tutorial_order: i32,
    pub page_id: Vec<i32>,
    pub page_replace_condition_group_id: i32,
    pub replace_page_id: Vec<i32>,
    #[cfg(feature = "strict_json_fields")]
    pub group_name: String,
    pub tutorial_tip: bool,
    pub drop_id: i32,
    pub disable_drop_reward: bool,
    pub require_read_all: bool,
    pub exclude_from_wiki: bool,
}