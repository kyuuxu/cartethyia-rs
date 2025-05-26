use serde::Deserialize;
use crate::ConsumeItem;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SynthesisFormulaData {
    pub id: i32,
    pub formula_item_id: i32,
    pub item_id: i32,
    pub formula_type: i32,
    pub item_group: i32,
    #[cfg(feature = "strict_json_fields")]
    pub name: String,
    pub consume_items: Vec<ConsumeItem>,
    pub sort_id: i32,
    pub unlock_condition: i32,
    pub proficiency: i32,
    pub max_proficiency_count: i32,
    pub type_id: i32,
    pub unlock: bool,
    pub limit_count: i32,
    pub permanent_limit: bool,
    pub role_list: Vec<i32>,
    #[cfg(feature = "strict_json_fields")]
    pub compose_content: String,
    #[cfg(feature = "strict_json_fields")]
    pub compose_background: String,
}