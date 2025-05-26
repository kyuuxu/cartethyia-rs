use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TeleportPosition {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
    pub a: Option<f32>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TeleportComponent {
    pub disabled: Option<bool>,
    pub teleporter_id: Option<i32>,
    #[serde(rename = "TeleportPos")]
    pub teleport_position: Option<TeleportPosition>,
}