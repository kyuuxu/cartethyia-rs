use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct FlySkinConfigData {
    pub id: i32,
    pub skin_type: i32,
    #[cfg(feature = "strict_json_fields")]
    pub model_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub stand_anim: String,
    pub quality_id: i32,
    pub skin_grade: i32,
    #[cfg(feature = "strict_json_fields")]
    pub name: String,
    #[cfg(feature = "strict_json_fields")]
    pub type_description: String,
    #[cfg(feature = "strict_json_fields")]
    pub attributes_description: String,
    #[cfg(feature = "strict_json_fields")]
    pub bg_description: String,
    #[cfg(feature = "strict_json_fields")]
    pub icon: String,
    #[cfg(feature = "strict_json_fields")]
    pub icon_middle: String,
    #[cfg(feature = "strict_json_fields")]
    pub icon_small: String,
    #[cfg(feature = "strict_json_fields")]
    pub mesh: String,
    #[cfg(feature = "strict_json_fields")]
    pub obtained_show_description: String,
    #[cfg(feature = "strict_json_fields")]
    pub show_in_bag: bool,
    #[cfg(feature = "strict_json_fields")]
    pub item_access: Vec<i32>,
    #[cfg(feature = "strict_json_fields")]
    pub sort_index: i32,
    #[cfg(feature = "strict_json_fields")]
    pub red_dot_disable_rule: i32,
    #[cfg(feature = "strict_json_fields")]
    pub preview_texture_in_buy_view: String,
    #[cfg(feature = "strict_json_fields")]
    pub preview_texture_in_pay_shop: String,
    #[cfg(feature = "strict_json_fields")]
    pub preview_texture_in_pop: String,
    #[cfg(feature = "strict_json_fields")]
    pub skin_obtain_color1: String,
    #[cfg(feature = "strict_json_fields")]
    pub skin_obtain_color2: String,
    #[cfg(feature = "strict_json_fields")]
    pub skin_obtain_image: String,
    #[cfg(feature = "strict_json_fields")]
    pub is_special_view_after_obtain: bool,
}
