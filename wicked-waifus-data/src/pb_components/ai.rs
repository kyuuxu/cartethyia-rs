use serde::Deserialize;
use crate::pb_components::action::Actions;
use crate::pb_components::condition::Conditions;

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct InitState {
    pub wander: Option<i32>,
    pub standby_tags: Option<Vec<String>>,
    pub r#type: Option<i32>,
    pub birth_tag: Option<String>,
    pub conditions: Option<Vec<Conditions>>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Patrol {
    pub is_circle: Option<bool>,
    pub spline_entity_id: Option<i64>,
    pub actions: Option<Vec<Actions>>
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EntityPos {
    pub key: String,
    pub entity_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EntityId {
    pub key: String,
    pub entity_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum BlackBoard {
    EntityPos(EntityPos),
    EntityId(EntityId),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AiComponent {
    pub disabled: Option<bool>,
    pub ai_id: Option<i32>,
    pub ai_team_level_id: Option<i32>,
    pub center_point: Option<i32>,
    pub init_black_board: Option<Vec<BlackBoard>>,
    pub init_state: Option<InitState>,
    pub patrol: Option<Patrol>,
    pub weapon_id: Option<String>,
}