use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct BossViewConfig {
    pub boss_state_view_type: Option<i32>,
    pub boss_state_info_show_type: Option<i32>,
    pub tid_level_text: Option<String>,
    pub tid_boss_sub_title: Option<String>,
    pub show_distance: Option<i32>,
    pub only_show_in_battle_state: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ShowOnDeath {
    pub r#type: Option<i32>,
    pub effect_id: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PerformConfig {
    pub show_on_death: Option<ShowOnDeath>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct MonsterComponent {
    pub disabled: Option<bool>,
    pub init_gas_tag: Option<Vec<String>>,
    pub special_hate_and_sense_config: Option<i32>,
    pub boss_view_config: Option<BossViewConfig>,
    pub perform_config: Option<PerformConfig>,
    pub fight_config_id: Option<i32>,
}