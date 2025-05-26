use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct FavorWordData {
    pub id: i32,
    pub role_id: i32,
    pub r#type: i32,
    pub sort: i32,
    #[cfg(feature = "strict_json_fields")]
    pub title: String,
    #[cfg(feature = "strict_json_fields")]
    pub content: String,
    #[cfg(feature = "strict_json_fields")]
    pub voice: String,
    #[cfg(feature = "strict_json_fields")]
    #[serde(rename = "CVCn")]
    pub cv_cn: String,
    #[cfg(feature = "strict_json_fields")]
    #[serde(rename = "CVJp")]
    pub cv_jp: String,
    #[cfg(feature = "strict_json_fields")]
    #[serde(rename = "CVEn")]
    pub cv_en: String,
    #[cfg(feature = "strict_json_fields")]
    #[serde(rename = "CVKo")]
    pub cv_ko: String,
    pub cond_group_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub motion_img: String,
    #[cfg(feature = "strict_json_fields")]
    pub ani_blueprint: String,
    #[cfg(feature = "strict_json_fields")]
    pub ani_montage: String,
}