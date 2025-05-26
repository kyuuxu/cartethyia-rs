use serde::Deserialize;
use crate::pb_components::condition::Conditions;
use crate::pb_components::interact::{Actions, Flows};

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SitDown {}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum OptionType {
    Actions(Actions),
    Flow(Flows),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GameOption {
    pub tid_content: Option<String>,
    pub guid: Option<String>,
    pub uniqueness_test: Option<String>,
    pub r#type: Option<OptionType>,
    pub condition: Option<Conditions>,
    pub icon: Option<String>
}