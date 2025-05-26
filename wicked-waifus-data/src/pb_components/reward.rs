use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RewardComponent {
    pub disabled: Option<bool>,
    pub reward_id: Option<i32>,
    pub reward_type: Option<i32>,
    pub drop_on_event: Option<i32>,
}