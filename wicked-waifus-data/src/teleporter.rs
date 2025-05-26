use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TeleporterData {
    pub id: i32,
    pub map_id: i32,
    pub object_id: i32,
    pub fog_id: i32,
    pub r#type: i32,
    pub teleport_entity_config_id: i64,
    #[cfg(feature = "strict_json_fields")]
    pub plot: String,
    pub after_network_action: i32,
    pub show_world_map: bool,
}

