use serde::Deserialize;
use crate::pb_components::ComponentsData;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TemplateConfigData {
    pub id: i32,
    pub blueprint_type: String,
    #[cfg(feature = "strict_json_fields")]
    pub name: String,
    pub components_data: ComponentsData,
}