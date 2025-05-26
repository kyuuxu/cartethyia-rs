use serde::Deserialize;
use crate::RawVectorData;
use crate::pb_components::ComponentsData;

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct LevelEntityConfigData {
    pub id: i32,
    pub map_id: i32,
    pub entity_id: i64,
    pub blueprint_type: String,
    #[cfg(feature = "strict_json_fields")]
    pub name: String,
    pub in_sleep: bool,
    pub is_hidden: bool,
    pub area_id: i32,
    pub transform: Vec<RawVectorData>,
    pub components_data: ComponentsData,
}
