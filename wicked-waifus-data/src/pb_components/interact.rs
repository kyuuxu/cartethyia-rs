use serde::Deserialize;
use crate::pb_components::action::Action;
use crate::pb_components::common::Point;
use crate::pb_components::flow::Flow;
use crate::pb_components::option::GameOption;

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct MatchRoleOption {
    pub r#type: String,
    pub id: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SectorRange {
    pub begin: Option<f32>,
    pub end: Option<f32>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RandomInteract {
    pub random_count: Option<i32>,
    #[cfg(feature = "strict_json_fields")]
    pub options: Option<serde_json::Value>, // TODO
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Npc {}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct LeisureInteraction {
    pub begin: i32,
    pub end: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum SectorRangeFromPlayerToEntity {
    Npc(Npc),
    LeisureInteraction(LeisureInteraction),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Actions {
    pub actions: Vec<Action>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Flows {
    pub flow: Flow,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct InteractComponent {
    pub disabled: Option<bool>,
    pub pre_flow: Option<Flow>,
    pub do_intact_type: Option<String>,
    pub options: Option<Vec<GameOption>>,
    pub match_role_option: Option<Vec<MatchRoleOption>>,
    pub range: Option<i32>,
    pub is_wait_for_turn_around_complete: Option<bool>,
    pub turn_around_type: Option<String>,
    pub interact_default_icon: Option<String>,
    pub tid_content: Option<String>,
    pub interact_point_offset: Option<Point>,
    pub sector_range: Option<SectorRange>,
    pub random_interact: Option<RandomInteract>,
    pub sector_range_from_player_to_entity: Option<SectorRangeFromPlayerToEntity>,
    pub interact_icon: Option<String>,
    pub quest_ids: Option<Vec<i32>>,
    pub exit_range: Option<i32>,
}