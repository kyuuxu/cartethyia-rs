use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Category {
    pub main_type: Option<String>,
    pub monster_match_type: Option<i32>,
    pub trace_match_type: Option<i32>,
    pub entity_plot_binding_type: Option<String>,
    pub exploratory_degree: Option<i32>,
    pub hide_in_flow_type: Option<i32>,
    pub destructible_type: Option<String>,
    pub vehicle_type: Option<String>,
    pub npc_type: Option<i32>,
    pub animal_type: Option<i32>, // non use except of null so type not exact
    pub collect_type: Option<String>,
    pub pull_statue_match_type: Option<String>,
    pub control_match_type: Option<String>,
    pub item_foundation: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TraceEffect {
    pub target: Option<i32>,
    pub effect: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ScanFunction {
    pub scan_id: Option<i32>,
    pub is_concealed: Option<bool>,
    pub trace_effect: Option<TraceEffect>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct HeadStateViewConfig {
    pub head_state_view_type: Option<i32>,
    #[serde(rename = "ZOffset")]
    pub z_offset: Option<i32>,
    pub forward_offset: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CustomAoiZRadius {
    pub up: Option<i32>,
    pub down: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct BaseInfoComponent {
    pub disabled: Option<bool>,
    pub tid_name: Option<String>,
    pub category: Option<Category>,
    pub camp: Option<i32>,
    pub head_info: Option<i32>,
    pub pack_id: Option<i32>,
    pub data_layers: Option<Vec<i32>>,
    pub is_show_name_on_head: Option<bool>,
    pub head_state_view_config: Option<HeadStateViewConfig>,
    #[cfg(feature = "strict_json_fields")]
    pub fix_processor: Option<serde_json::Value>, // TODO: Implement this shit
    pub online_interact_type: Option<i32>,
    pub scan_function: Option<ScanFunction>,
    pub aoi_layer: Option<i32>,
    pub entity_property_id: Option<i32>,
    pub focus_priority: Option<i32>,
    #[serde(rename = "AoiZRadius")]
    pub aoi_zradius: Option<i32>,
    #[serde(rename = "CustomAoiZRadius")]
    pub custom_aoi_zradius: Option<CustomAoiZRadius>,
    pub child_entity_ids: Option<Vec<i64>>,
    pub not_allow_hided_by_target_range: Option<bool>,
    pub occupation: Option<String>,
    pub is_online_standalone: Option<bool>,
    pub entity_update_strategy: Option<i32>,
    pub specified_online_standalone_parent_uids: Option<Vec<String>>,
    pub lower_npc_density: Option<i32>,
    pub map_icon: Option<i32>,
}
