use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct WeaponSkinData {
    pub id: i32,
    pub weapon_skin_type: i32,
    #[cfg(feature = "strict_json_fields")]
    pub name: String,
    pub hide_in_skin_view: bool,
    pub quality_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub model_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub transform_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub models: Vec<i32>,
    #[cfg(feature = "strict_json_fields")]
    pub type_description: String,
    #[cfg(feature = "strict_json_fields")]
    pub attributes_description: String,
    #[cfg(feature = "strict_json_fields")]
    pub bg_description: String,
    #[cfg(feature = "strict_json_fields")]
    pub card_icon_path: String,
    #[cfg(feature = "strict_json_fields")]
    pub icon: String,
    #[cfg(feature = "strict_json_fields")]
    pub icon_middle: String,
    #[cfg(feature = "strict_json_fields")]
    pub icon_small: String,
    #[cfg(feature = "strict_json_fields")]
    #[serde(rename = "MaxCapcity")] // kuro!
    pub max_capacity: i32,
    #[cfg(feature = "strict_json_fields")]
    pub item_access: Vec<i32>,
    #[cfg(feature = "strict_json_fields")]
    pub obtained_show: i32,
    #[cfg(feature = "strict_json_fields")]
    pub obtained_show_description: String,
    #[cfg(feature = "strict_json_fields")]
    pub num_limit: i32,
    #[cfg(feature = "strict_json_fields")]
    pub show_in_bag: bool,
    #[cfg(feature = "strict_json_fields")]
    pub sort_index: i32,
    #[cfg(feature = "strict_json_fields")]
    pub hidden_time: i32,
    #[cfg(feature = "strict_json_fields")]
    pub destructible: bool,
    #[cfg(feature = "strict_json_fields")]
    pub red_dot_disable_rule: i32,
}
