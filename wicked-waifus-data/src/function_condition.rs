use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct FunctionConditionData {
    pub function_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub name: String,
    pub is_on: bool,
    pub open_condition_id: i32,
    #[cfg(feature = "strict_json_fields")]
    #[serde(rename = "ShowUIType")]
    pub show_ui_type: i32,
    #[cfg(feature = "strict_json_fields")]
    pub delay: i32,
    #[cfg(feature = "strict_json_fields")]
    pub title: String,
    #[cfg(feature = "strict_json_fields")]
    pub desc: String,
    #[cfg(feature = "strict_json_fields")]
    pub icon: String,
    #[cfg(feature = "strict_json_fields")]
    pub icon_sprite: String,
}
