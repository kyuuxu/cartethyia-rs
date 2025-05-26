use serde::Deserialize;

use crate::pb_components::action::Action;
use crate::pb_components::common::Location;
use crate::pb_components::condition::{Condition, Conditions};

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TrackTypeLocations {
    pub locations: Vec<Location>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TrackTypeEntities {
    pub entity_ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TrackTypeCaptureVisions {
    pub vision_drop_entities: Vec<i64>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "Type")]
pub enum TrackType {
    Locations(TrackTypeLocations),
    Entities(TrackTypeEntities),
    CaptureVisions(TrackTypeCaptureVisions),
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TrackTarget {
    pub track_type: TrackType,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NodeDataDetailCommon {
    pub id: i32,
    // In ps we don't care about description
    #[cfg(feature = "strict_json_fields")]
    pub desc: String,
    // ParentNodeId is not defined for child items. i.e.: QuestSucceed/ConditionSelector sub items
    pub parent_node_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct PosRollbackCfg {
    pub range_entity_id: i64,
    pub position_entity_id: i64,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SaveConfig {
    pub disable_rollback_while_reconnect: Option<bool>,
    pub enable_rollback_while_switch_online_mode: Option<bool>,
    pub enter_actions: Option<Vec<Action>>,
    pub pos_rollback_cfg: Option<PosRollbackCfg>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NodeDataDetailSequence {
    #[serde(flatten)]
    pub common: NodeDataDetailCommon,
    pub children: Option<Vec<NodeDataDetail>>,
    #[cfg(feature = "strict_json_fields")]
    pub save_config: Option<SaveConfig>,
    #[cfg(feature = "strict_json_fields")]
    #[serde(rename = "UIConfig")]
    pub ui_config: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub information_view: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub statistics: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub enable_system: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub special_game_play_config: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub reward_config: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub fix_processor: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub budget_camera_type: Option<serde_json::Value>,
    pub disable_online: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NodeDataDetailChildQuest {
    #[serde(flatten)]
    pub common: NodeDataDetailCommon,
    pub condition: Option<Condition>,
    // In ps we don't care about tid tip
    #[cfg(feature = "strict_json_fields")]
    pub tid_tip: Option<String>,
    pub track_target: Option<TrackTarget>,
    pub enter_actions: Option<Vec<Action>>,
    pub finish_actions: Option<Vec<Action>>,
    #[cfg(feature = "strict_json_fields")]
    pub fix_processor: Option<serde_json::Value>,
    pub reward_id: Option<i32>,
    // In ps we don't care about this
    #[cfg(feature = "strict_json_fields")]
    pub reward_get_ui_type: Option<i32>,
    // In ps we don't care about this
    #[cfg(feature = "strict_json_fields")]
    pub show_tip_before_enter_actions: Option<bool>,
    // In ps we don't care about this
    #[cfg(feature = "strict_json_fields")]
    pub hide_ui: Option<bool>,
    // In ps we don't care about this
    #[cfg(feature = "strict_json_fields")]
    pub hide_tip: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NodeDataDetailSelect {
    #[serde(flatten)]
    pub common: NodeDataDetailCommon,
    pub children: Option<Vec<NodeDataDetail>>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NodeDataDetailParallelSelect {
    #[serde(flatten)]
    pub common: NodeDataDetailCommon,
    pub count: i32,
    pub children: Option<Vec<NodeDataDetail>>,
    #[cfg(feature = "strict_json_fields")]
    pub save_config: Option<SaveConfig>,
    #[cfg(feature = "strict_json_fields")]
    #[serde(rename = "UIConfig")]
    pub ui_config: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub statistics: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub occupation_config: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub start_wave: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub composite_track_view_mode: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub enable_system: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub information_view: Option<serde_json::Value>,
    pub disable_online: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ConditionSelectorSlot {
    pub condition: Conditions,
    pub node: Option<NodeDataDetail>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NodeDataDetailConditionSelector {
    #[serde(flatten)]
    pub common: NodeDataDetailCommon,
    pub slots: Vec<ConditionSelectorSlot>,
    #[cfg(feature = "strict_json_fields")]
    pub save_config: Option<SaveConfig>,
    #[cfg(feature = "strict_json_fields")]
    #[serde(rename = "UIConfig")]
    pub ui_config: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NodeDataDetailRepeater {
    #[serde(flatten)]
    pub common: NodeDataDetailCommon,
    pub max_repeat_times: Option<i32>,
    pub exit_condition: Option<Conditions>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NodeDataDetailCondition {
    #[serde(flatten)]
    pub common: NodeDataDetailCommon,
    pub condition: Conditions,
    pub wait_until_true: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NodeDataDetailAction {
    #[serde(flatten)]
    pub common: NodeDataDetailCommon,
    pub actions: Vec<Action>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NodeDataDetailActionWithResult {
    #[serde(flatten)]
    pub common: NodeDataDetailCommon,
    pub action: Action,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NodeDataDetailQuestSucceed {
    #[serde(flatten)]
    pub common: NodeDataDetailCommon,
    pub finish_actions: Option<Vec<Action>>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NodeDataDetailQuestFailed {
    #[serde(flatten)]
    pub common: NodeDataDetailCommon,
    #[cfg(feature = "strict_json_fields")]
    pub failed_condition: Option<serde_json::Value>,
    pub finish_actions: Option<Vec<Action>>,
    pub auto_failed: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NodeDataDetailAlways {
    #[serde(flatten)]
    pub common: NodeDataDetailCommon,
    pub child: Option<Box<NodeDataDetail>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "Type")]
pub enum NodeDataDetail {
    Sequence(NodeDataDetailSequence),
    ChildQuest(NodeDataDetailChildQuest),
    Select(NodeDataDetailSelect),
    ParallelSelect(NodeDataDetailParallelSelect),
    ConditionSelector(NodeDataDetailConditionSelector),
    Repeater(NodeDataDetailRepeater),
    Condition(NodeDataDetailCondition),
    Action(NodeDataDetailAction),
    ActionWithResult(NodeDataDetailActionWithResult),
    QuestSucceed(NodeDataDetailQuestSucceed),
    QuestFailed(NodeDataDetailQuestFailed),
    AlwaysFalse(NodeDataDetailAlways),
    AlwaysTrue(NodeDataDetailAlways),
}