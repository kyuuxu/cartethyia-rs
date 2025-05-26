use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DropPackageData {
    pub id: i32,
    pub show_bg: bool,
    #[cfg(feature = "strict_json_fields")]
    pub title: String,
    pub drop_plan: i32,
    pub drop_preview: HashMap<i32, i32>,
}