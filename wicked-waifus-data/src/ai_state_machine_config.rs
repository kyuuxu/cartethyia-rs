use serde::Deserialize;
use crate::StateMachineJson;

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AiStateMachineConfigData {
    pub id: String,
    pub state_machine_json: StateMachineJson,
}