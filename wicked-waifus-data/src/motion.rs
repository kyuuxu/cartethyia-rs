use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct MotionData {
    pub id: i32,
    pub role_id: i32,
    pub skin_id: i32,
    pub r#type: i32,
    pub sort: i32,
    #[cfg(feature = "strict_json_fields")]
    pub title: String,
    #[cfg(feature = "strict_json_fields")]
    pub content: String,
    #[cfg(feature = "strict_json_fields")]
    pub motion_img: String,
    #[cfg(feature = "strict_json_fields")]
    pub ani_blueprint: String,
    #[cfg(feature = "strict_json_fields")]
    pub ani_montage: String,
    pub cond_group_id: i32,
}