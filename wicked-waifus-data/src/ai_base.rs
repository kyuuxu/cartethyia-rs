use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SubBehaviorConfigs {
    #[cfg(feature = "strict_json_fields")]
    pub ai_alert: Option<String>,
    #[cfg(feature = "strict_json_fields")]
    pub ai_base_skill: Option<String>,
    #[cfg(feature = "strict_json_fields")]
    pub ai_battle_wander: Option<String>,
    #[cfg(feature = "strict_json_fields")]
    pub ai_flee: Option<String>,
    #[cfg(feature = "strict_json_fields")]
    pub ai_hate: Option<String>,
    #[cfg(feature = "strict_json_fields")]
    pub ai_patrol: Option<String>,
    #[cfg(feature = "strict_json_fields")]
    pub ai_sense: Option<String>,
    #[cfg(feature = "strict_json_fields")]
    pub ai_wander: Option<String>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AiBaseData {
    pub id: i32,
    pub state_machine: String,
    #[cfg(feature = "strict_json_fields")]
    pub ai_controller: String,
    #[cfg(feature = "strict_json_fields")]
    pub behavior_tree: String,
    #[cfg(feature = "strict_json_fields")]
    pub sub_behavior_configs: SubBehaviorConfigs,
    #[cfg(feature = "strict_json_fields")]
    pub team: bool,
    #[cfg(feature = "strict_json_fields")]
    pub monster_type: i32,
}