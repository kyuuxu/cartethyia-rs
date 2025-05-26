use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Flow {
    pub flow_list_name: String,
    pub flow_id: i32,
    pub state_id: Option<i32>,
    pub flow_guid: Option<String>
}
