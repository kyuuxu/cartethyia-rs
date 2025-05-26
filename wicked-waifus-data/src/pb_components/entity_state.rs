use serde::Deserialize;
use crate::pb_components::action::Action;
use crate::pb_components::condition::Conditions;

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ConditionAction {
    pub condition: Conditions,
    pub action: Vec<Action>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct DelayChangeState {
    pub time: f32,
    pub new_state: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct StateConfigs {
    pub duration: f32,
    pub state: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct StateChangeBehaviors {
    pub state: String,
    pub action: Option<Vec<Action>>,
    pub condition_action: Option<Vec<ConditionAction>>,
    pub delay_change_state: Option<DelayChangeState>
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LockConfig {
    pub lock_type: Option<String>, // TODO: Enum
    pub is_init_lock: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EntityStateComponent {
    pub disabled: Option<bool>,
    pub r#type: Option<String>,
    pub state: Option<String>,
    pub state_change_condition: Option<Conditions>,
    pub state_change_behaviors: Option<Vec<StateChangeBehaviors>>,
    pub state_configs: Option<Vec<StateConfigs>>,
    pub cycle_states: Option<Vec<String>>,
    pub prefab_performance_type: Option<String>,
    pub lock_config: Option<LockConfig>,
    pub instant_actions_on_state_change: Option<bool>,
}