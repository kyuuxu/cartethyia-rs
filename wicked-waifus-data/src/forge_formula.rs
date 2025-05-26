use serde::Deserialize;
use crate::ConsumeItem;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ForgeFormulaData {
    pub id: i32,
    pub formula_item_id: i32,
    pub item_id: i32,
    pub type_id: i32,
    pub unlock: bool,
    pub sort_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub name: String,
    pub consume_items: Vec<ConsumeItem>,
    #[cfg(feature = "strict_json_fields")]
    pub forge_content: String,
    #[cfg(feature = "strict_json_fields")]
    pub background: String,
    pub role_list: Vec<i32>,
}