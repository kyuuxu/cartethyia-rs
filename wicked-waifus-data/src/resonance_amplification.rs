use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ResonanceAmplificationData {
    pub id: i32,
    pub attr: i32,
    pub life_seat_boost_factor: Vec<f64>,
}
