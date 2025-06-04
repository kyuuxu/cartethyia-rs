use serde::Deserialize;

pub mod action;
pub mod ai;
pub mod attribute;
pub mod base_info;
pub mod common;
pub mod condition;
pub mod entity_state;
pub mod flow;
pub mod interact;
pub mod monster;
pub mod option;
pub mod reward;
pub mod teleport;
pub mod timer;
pub mod var;

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ComponentsData {
    pub base_info_component: Option<base_info::BaseInfoComponent>,
    pub ai_component: Option<ai::AiComponent>,
    pub attribute_component: Option<attribute::AttributeComponent>,
    pub teleport_component: Option<teleport::TeleportComponent>,
    pub monster_component: Option<monster::MonsterComponent>,
    pub interact_component: Option<interact::InteractComponent>,
    pub entity_state_component: Option<entity_state::EntityStateComponent>,
    pub reward_component: Option<reward::RewardComponent>,
    // TODO: Implement this ones
    #[cfg(feature = "strict_json_fields")]
    pub scene_actor_ref_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub effect_area_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub condition_listener_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub npc_perform_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub var_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub entity_visible_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub level_ai_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub trigger_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub range_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub spline_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub bubble_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub refresh_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub passerby_npc_spawn_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub vision_capture_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub refresh_group_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub collect_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub target_gear_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub fight_interact_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub guide_line_creator_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub photo_target_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub model_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub entity_group_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub scene_item_life_cycle_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub entity_state_audio_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub animal_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub nearby_tracking_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub follow_track_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub jigsaw_foundation: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub treasure_box_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub hook_lock_point: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub explore_skill_interact_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub attach_target_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub target_gear_group_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub spawn_monster_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub skybox_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub destructible_item: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub fan_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub state_hint_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub buff_consumer_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub reset_entities_pos_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub group_ai_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub pulling_object_foundation: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub lift_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub scene_item_movement_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub reset_self_pos_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub jigsaw_item: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub level_play_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub interact_gear_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub ai_gear_strategy_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub pick_interact_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub level_sequence_frame_event_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub air_wall_spawner_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub progress_bar_control_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub batch_bullet_caster_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub client_trigger_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub enrichment_area_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub vehicle_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub item_foundation2: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub tele_control2: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub interact_audio_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub level_qte_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub resurrection_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub ai_alert_notify_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub trample_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub dungeon_entry_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub level_prefab_perform_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub render_specified_range_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub walking_pattern_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub no_render_portal_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub adsorb_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub beam_cast_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub beam_receive_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub timeline_track_control_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub scene_bullet_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub edit_custom_aoi_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub combat_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub location_safety_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub turntable_control_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub scene_item_ai_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub buff_producer_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub portal_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub inhalation_ability_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub inhaled_item_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub monster_gacha_base_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub monster_gacha_item_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub time_stop_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub hit_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub levitate_magnet_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub rebound_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub rotator_component2: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub conveyor_belt_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub dynamic_portal_creator_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub connector_component: Option<serde_json::Value>,
    #[cfg(feature = "strict_json_fields")]
    pub monitor_component: Option<serde_json::Value>,
}

impl ComponentsData {
    pub fn merge_with_template(&self, template: &Self) -> Self {
        Self {
            base_info_component: self.base_info_component.as_ref().or(template.base_info_component.as_ref()).cloned(),
            // TODO: LevelEntityConfig doesn't have an AI ID,
            // but TemplateConfig does.
            // So for now I'll use from TemplateConfig
            ai_component: template.ai_component.as_ref().cloned(),
            attribute_component: self.attribute_component.as_ref().or(template.attribute_component.as_ref()).cloned(),
            teleport_component: self.teleport_component.as_ref().or(template.teleport_component.as_ref()).cloned(),
            monster_component: self.monster_component.as_ref().or(template.monster_component.as_ref()).cloned(),
            interact_component: self.interact_component.as_ref().or(template.interact_component.as_ref()).cloned(),
            entity_state_component: self.entity_state_component.as_ref().or(template.entity_state_component.as_ref()).cloned(),
            reward_component: self.reward_component.as_ref().or(template.reward_component.as_ref()).cloned(),
        }
    }
}