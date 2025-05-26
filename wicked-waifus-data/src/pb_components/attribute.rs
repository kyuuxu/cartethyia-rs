use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct WorldLevelBonusType {
    pub r#type: Option<i32>,
    pub world_level_bonus_id: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct FightMusic {
    pub fight_music: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct FightMusics {
    pub r#type: Option<String>,
    pub element: Option<Vec<FightMusic>>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AttributeComponent {
    pub disabled: Option<bool>,
    pub append_buff_ids: Option<Vec<i64>>,
    pub property_id: Option<i32>,
    pub level: Option<i32>,
    pub world_level_bonus_type: Option<WorldLevelBonusType>,
    pub rage_mode_id: Option<i32>,
    pub hardness_mode_id: Option<i32>,
    pub monster_prop_extra_rate_id: Option<i32>,
    pub world_level_bonus_id: Option<i32>,
    pub fight_music: Option<String>,
    pub fight_musics: Option<FightMusics>,
}