use serde::Deserialize;

use crate::pb_components::common::Point;
use crate::pb_components::flow::Flow;
use crate::pb_components::teleport::TeleportPosition;
use crate::pb_components::timer::TimerType;
use crate::pb_components::var::{OpType, Var};

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NotifyMonsterPlayStandbyTags {
    pub standby_tags: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum SetType {
    Add,
    Remove,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct BattleTag {
    pub entity_id: i64,
    pub set_type: SetType,
    pub gameplay_tag: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetBattleTag {
    pub set_tags: Vec<BattleTag>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NotifyGatherToEntity {
    pub entity_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum PerceptionBehaviorOption {
    NotifyGatherToEntity(NotifyGatherToEntity),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NotifyMonsterPerception {
    pub perception_behavior_option: PerceptionBehaviorOption,
    pub entity_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum StateOption {
    NotifyMonsterPlayStandbyTags(NotifyMonsterPlayStandbyTags),
    SetBattleTag(SetBattleTag),
    NotifyMonsterPerception(NotifyMonsterPerception),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetBattleStateParams {
    pub state_option: StateOption,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct BattleTags {
    pub entity_id: i64,
    pub tag_config_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetBattleTags {
    pub configs: Vec<BattleTags>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetMonsterMoveTarget {
    pub target_entity_id: i64,
    pub monster_entity_ids: Vec<i64>,
    pub move_event: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum BattleOption {
    SetBattleTags(SetBattleTags),
    SetMonsterMoveTarget(SetMonsterMoveTarget),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ExecBattleAction {
    pub battle_option: BattleOption,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct WaitBattleCondition {
    // TODO: Implement this one
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ActionFields<T> {
    pub params: T,
    pub action_guid: Option<String>,
    pub action_id: Option<i32>,
    pub r#async: Option<bool>,
    pub disabled: Option<bool>,
    pub finish_send_self_event: Option<String>, // TODO: Enum??
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CollectParams {
    pub target_entity: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct LeisureInteractOptionBounce {
    pub height: i64,
    pub time: f32,
    pub motion_curve: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct LeisureInteractOptionCatapultParams {
    pub p1: Point,
    pub p2: Point,
    pub time: Option<f32>,
    pub gravity: Option<i32>,
    pub motion_curve: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct LeisureInteractOptionCatapult {
    pub param: LeisureInteractOptionCatapultParams,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct LeisureInteractOptionHookLock {
    pub entity_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct LeisureInteractOptionManipulate {
    pub target_entity_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum LeisureInteractOption {
    SitDown,
    SitOnGround,
    StandControl,
    StandControl2,
    Bounce(LeisureInteractOptionBounce),
    Soar,
    Catapult(LeisureInteractOptionCatapult),
    SuperCatapult(LeisureInteractOptionCatapult),
    HookLock(LeisureInteractOptionHookLock),
    Manipulate(LeisureInteractOptionManipulate),
    Glide,
    GetUp,
    FailurePose,
    GameplayPose1,
    GameplayPose2,
    GameplayPose3,
    FaithJump
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct LeisureInteract {
    pub option: LeisureInteractOption,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct UnlockTeleportTrigger {
    pub teleport_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EnableTemporaryTeleport {
    pub enable: bool,
    pub range_entity: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct OpenSystemBoard {
    pub system_type: String,
    pub board_id: i32,
    pub gramophone_id: Option<i32>,
    pub action_montage: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct OpenSystemFunction {
    pub function_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ChangeSelfEntityState {
    pub entity_state: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PlayerOperationRestrictionEnableAll {}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PlayerOperationRestrictionDisableAll {
    pub display_mode: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DisableExploreSkill {
    pub place_temporary_teleport: Option<bool>,
    pub explore_skill_list: Option<Vec<i32>>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SkillOptionDisableSection {
    pub display_mode: String,
    pub disable_explore_skill: Option<DisableExploreSkill>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SkillOptionDisable {
    pub display_mode: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum SkillOption {
    DisableSection(SkillOptionDisableSection),
    Disable(SkillOptionDisable),
    Enable,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct MoveOptionDisable {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub force_walk: Option<bool>,
    pub forbid_sprint: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum MoveOption {
    Disable(MoveOptionDisable),
    Enable,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum CameraOption {
    Disable,
    Enable,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EnableSectionalUi {
    #[serde(default)]
    pub show_mini_map: bool,
    #[serde(default)]
    pub show_quest_track: bool,
    #[serde(default)]
    pub show_esc: bool,
    #[serde(default)]
    pub show_system: bool,
    #[serde(default)]
    pub show_screen_effect: bool,
    #[serde(default)]
    pub show_other: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum UiOption {
    Disable,
    Enable,
    EnableSectionalUi(EnableSectionalUi),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum SceneInteractionOption {
    Disable,
    Enable,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PlayerOperationRestrictionDisableModule {
    pub skill_option: Option<SkillOption>,
    pub move_option: Option<MoveOption>,
    pub camera_option: Option<CameraOption>,
    pub ui_option: Option<UiOption>,
    pub scene_interaction_option: Option<SceneInteractionOption>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum SetPlayerOperationRestriction {
    EnableAll(PlayerOperationRestrictionEnableAll),
    DisableAll(PlayerOperationRestrictionDisableAll),
    DisableModule(PlayerOperationRestrictionDisableModule),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Wait {
    pub time: f32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ChangeEntityStateDirectly {
    pub entity_id: i64,
    pub state: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ChangeEntityStateBatchDirectly {
    pub entity_id: i64,
    pub entity_ids: Vec<i64>,
    pub state: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ChangeEntityStateLoop {
    pub entity_id: i64,
    pub circulation: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum ChangeEntityState {
    Directly(ChangeEntityStateDirectly),
    BatchDirectly(ChangeEntityStateBatchDirectly),
    Loop(ChangeEntityStateLoop),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Log {
    pub level: String,
    pub content: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EnableNearbyTracking {
    pub is_enable: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CenterTextFlow {
    pub flow_list_name: String,
    pub flow_id: i32,
    pub state_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TransitionOptionCenterText {
    pub center_text_flow: CenterTextFlow,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TransitionOptionFadeInScreen {}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TransitionOptionSeamless {
    pub effect_da_path: String,
    pub least_time: i32,
    pub effect_expand_time: i32,
    pub effect_collapse_time: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TransitionOptionPlayMp4 {
    #[serde(rename = "Mp4Path")]
    pub mp4_path: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TransitionOptionCharacterDisplay {
    // TODO:
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum TransitionOption { // TODO: Extract
    CenterText(TransitionOptionCenterText),
    FadeInScreen(TransitionOptionFadeInScreen),
    Seamless(TransitionOptionSeamless),
    PlayMp4(TransitionOptionPlayMp4),
    CharacterDisplay(TransitionOptionCharacterDisplay),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TeleportDungeon {
    pub is_regroup: bool,
    pub dungeon_id: i32,
    pub location_entity_id: Option<i64>,
    pub transition_option: Option<TransitionOption>,
    pub is_need_secondary_confirmation: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DestroySelf {}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CameraLookAt {
    pub pos: Point,
    pub fade_in_time: f32,
    pub stay_time: f32,
    pub fade_out_time: f32,
    pub lock_camera: Option<bool>,
    pub cancel_buffer: Option<bool>,
    pub cancel_blend_out: Option<bool>,
    pub ban_input: Option<bool>,
    pub camera_pos: Option<Point>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StopCameraLookAt {}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EnterOrbitalCamera {
    // TODO: Implement this one
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ExitOrbitalCamera {
    // TODO: Implement this one
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SendAiEvent {
    pub event_type: String, // TODO: Check if enum
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetInteractionLockState {
    pub is_lock: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AwakeEntity {
    pub entity_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ChangeLiftTarget {
    pub is_self: bool,
    pub location: i32,
    pub entity_id: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CalculateVar {
    pub var1: Var,
    pub var2: Var,
    pub op: OpType,
    pub result: Var,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AddBuffToPlayer {
    pub buff_ids: Vec<i64>,
    pub persist_on_destroy_buff_ids: Option<Vec<i64>>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RemoveBuffFromPlayer {
    pub buff_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SingleEntityBuffs {
    pub entity_id: i64,
    pub buff_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct MultipleEntitiesBuff {
    pub entity_ids: Vec<i64>,
    pub buff_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SelfEntityBuff {
    pub buff_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum AddBuffToEntity {
    SingleEntityBuffs(SingleEntityBuffs),
    MultipleEntitiesBuff(MultipleEntitiesBuff),
    SelfEntityBuff(SelfEntityBuff),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum RemoveBuffFromEntity {
    SingleEntityBuffs(SingleEntityBuffs),
    MultipleEntitiesBuff(MultipleEntitiesBuff),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Prompt {
    pub general_text_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetEntityVisible {
    pub visible: bool,
    pub entity_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DestroyEntity {
    pub entity_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SimpleGuide {
    pub guide_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum GuideTypes {
    BeginnerGuide(SimpleGuide),
    AttackGuide(SimpleGuide),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CameraShakeConstant {}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CameraShakeLinearOverRange {
    pub center_entity_id: i64,
    pub min_range: i32,
    pub max_range: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum CameraShakeConfig {
    Constant(CameraShakeConstant),
    LinearOverRange(CameraShakeLinearOverRange),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TriggerCameraShake {
    pub camera_shake_bp: String,
    pub camera_shake_config: CameraShakeConfig,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetVar {
    pub var_left: Var,
    pub var_right: Var,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct VehicleEnterTypePlayer {
    pub target_vehicle: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct VehicleEnterTypeNpc {
    pub target_npc: i64,
    pub target_vehicle: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum VehicleEnterType {
    Player(VehicleEnterTypePlayer),
    Npc(VehicleEnterTypeNpc),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct VehicleEnter {
    pub entering_target: VehicleEnterType,
    pub seat: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct VehicleExitPlayer {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct LockEntity {
    pub entity_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct UnlockEntity {
    pub entity_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TipOption {
    pub r#type: i32,
    pub tid_text: Option<String>,
    pub tid_main_text: Option<String>,
    pub tid_sub_text: Option<String>,
    pub main_text: Option<String>,
    pub sub_text: Option<String>,
    pub warning_text: Option<String>,
    pub id: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CommonTip {
    pub tip_option: TipOption,
    pub duration: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TipOption2 {
    pub r#type: i32,
    pub count_down_num: i32,
    pub tid_count_down_txt: String,
    pub is_block_player: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CommonTip2 {
    pub tip_option: TipOption2,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EventConfigGlobal {
    pub ak_event: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EventConfigTarget {
    pub ak_event: String,
    pub entity_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum EventConfig {
    Global(EventConfigGlobal),
    Target(EventConfigTarget),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PostAkEvent {
    pub event_config: EventConfig,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct VehicleEnterNpc {
    pub target: i32,
    pub seat: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ExitTypeTeleport {
    pub pos: Point,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum ExitType {
    Teleport(ExitTypeTeleport)
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct VehicleExitNpc {
    pub target_npc: i32,
    pub exit_type: ExitType,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PlayerLookAt {
    pub pos: Point,
    pub camera_move: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PlayBubble {
    pub entity_id: i64,
    pub flow: Flow,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AddPlayBubble {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ClearPlayBubble {
    pub entity_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EnableLevelPlayConfig {
    pub level_play_id: i64,
    pub enable: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EnableLevelPlay {
    pub configs: Vec<EnableLevelPlayConfig>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ClaimLevelPlayReward {}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ExecRiskHarvestEffect {
    pub id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SettlementDungeon {}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ExitDungeon {
    pub is_need_secondary_confirmation: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct FinishDungeon {
    pub is_success: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DungeonEventConfig {
    pub is_success: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub enum TimeStampType {
    DungeonBegin,
    DungeonFinish,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RecordTimeStamp {
    pub time_stamp_type: TimeStampType,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum RecordDungeonEventConfig {
    RecordTimeStamp(RecordTimeStamp)
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RecordDungeonEvent {
    pub event_config: RecordDungeonEventConfig,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RecoverDurability {
    pub entity_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Ease {
    pub r#type: i32,
    pub duration: f32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct FadeInScreen {
    pub ease: Option<Ease>,
    pub screen_type: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct FadeOutScreen {
    pub ease: Option<Ease>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ChangeNpcPerformState {
    pub entity_id: i64,
    pub state: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EntityTurnToTarget {
    pub r#type: i32, // Type 2 has entity id, Type 3 has pos, type 4 has nothing
    pub entity_id: Option<i64>,
    pub pos: Option<Point>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EntityTurnTo {
    pub entity_id: i64,
    pub target: EntityTurnToTarget,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EntityLookAt {
    pub entity_id: i64,
    pub pos: Point,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ToggleMapMarkStateShow {
    pub mark_id: i64,
    pub is_focus_on_first_show: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ToggleMapMarkStateHide {
    pub mark_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum ToggleMapMarkState {
    Show(ToggleMapMarkStateShow),
    Hide(ToggleMapMarkStateHide),
    Disable(ToggleMapMarkStateHide),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RandomVar {
    pub left_var: Var,
    pub right_var: Var,
    pub result: Var,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ModifySceneItemAttributeTagTarget {
    pub is_add_tag: bool,
    pub entity_ids: Vec<i64>,
    pub performance_tag: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ModifySceneItemAttributeTagSelf {
    pub is_add_tag: bool,
    pub performance_tag: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum ModifySceneItemAttributeTag {
    Target(ModifySceneItemAttributeTagTarget),
    #[serde(rename = "Self")]
    SelfType(ModifySceneItemAttributeTagSelf),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct VehicleWaterfallClimbing {
    pub spline_entity_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AppointedVehicleType {
    pub vehicle_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum VehicleType {
    FishingBoat,
    Gongduola,
    Appointed(AppointedVehicleType),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct VehicleTeleport {
    pub target_vehicle: VehicleType,
    pub pos: Point,
    pub rot: Option<Point>,
    pub appointed_dock: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RogueGotoNextFloor {}

#[derive(Deserialize, Debug, Clone)]
pub enum RogueRewardReceiveType {
    ReceiveByEnergy
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RogueReceiveReward {
    pub rogue_reward_receive_type: Option<RogueRewardReceiveType>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum RogueSelectRoomConfigType {
    Role
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RogueSelectRoomConfig {
    pub r#type: RogueSelectRoomConfigType,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RogueSelectRoom {
    pub config: RogueSelectRoomConfig,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RogueActivatePortal {}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct MowingTowerGotoNextFloor {}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SlashAndTowerGotoNextFloor {}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ActionMontageNormal {
    pub path: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "MontageType")]
pub enum ActionMontage {
    Normal(ActionMontageNormal),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PlayMontage {
    pub entity_id: i64,
    pub action_montage: ActionMontage,
    pub duration: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SystemTypeConfirmBox {
    pub id: i64,
    pub return_var: Var,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SystemTypeFishingItemDelivery {
    pub preset_id: i64,
    pub return_var: Var,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SystemTypeSoaringChallenge {
    pub score: Var,
    pub rank_s: i32,
    pub rank_a: i32,
    pub rank_b: i32,
    pub return_var: Var,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum SystemType {
    ConfirmBox(SystemTypeConfirmBox),
    FishingItemDelivery(SystemTypeFishingItemDelivery),
    SoaringChallenge(SystemTypeSoaringChallenge),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct OpenSystemBoardWithReturn {
    pub system_type: SystemType,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SystemOptionAchievementSystem {
    pub id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct UnlockCookBook {
    pub cook_book_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GeographicalAtlas {
    pub id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PlotPhoto {
    pub id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Noun {
    pub id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum UnlockOption {
    UnlockCookBook(UnlockCookBook),
    GeographicalAtlas(GeographicalAtlas),
    PlotPhoto(PlotPhoto),
    Noun(Noun),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SystemOptionUnlockOption {
    pub unlock_option: UnlockOption,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct MemoirsSystem {
    pub id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DangoCollect {
    pub id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum SystemOption {
    AchievementSystem(SystemOptionAchievementSystem),
    CookSystem(SystemOptionUnlockOption),
    AtlasSystem(SystemOptionUnlockOption),
    MemoirsSystem(MemoirsSystem),
    DangoCollect(DangoCollect),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct UnlockSystemItem {
    pub system_option: SystemOption,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SportsStateConfigSlide {
    pub slide_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SportsStateConfigSki {
    // TODO: Implement this one
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum SportsStateConfig {
    Slide(SportsStateConfigSlide),
    Ski(SportsStateConfigSki),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetSportsState {
    pub config: SportsStateConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Color {
    Red,
    Blue,
    Yellow,
    Green,
    White,
    Gray
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ColorConfig {
    pub color: Color,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GameplayConfigSignalDevice {
    pub config: Vec<ColorConfig>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GameplayConfigSignalBreak {
    pub signal_break_id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GameplayConfigRenjuChess {
    pub chessboard: i64,
    // TODO:Implement this
    #[cfg(feature = "strict_json_fields")]
    pub camera_config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ColorBoard {
    pub config: Vec<ColorConfig>,
    pub colors: Vec<Color>,
    pub target_color: Color,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct MaxStepRewardRule {
    pub paint_count: i32,
    pub add_step: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GameplayConfigLifePoint {
    pub color_board: ColorBoard,
    pub step_limit: i32,
    pub tid_desc: String,
    pub max_step_reward_rule: Vec<MaxStepRewardRule>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GameplayConfigMorseCode {
    pub morse_code_id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GameplayConfigCipher {
    pub cipher_id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GameplayConfigBrokenRock {
    pub id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum GameplayConfig {
    SignalDevice(GameplayConfigSignalDevice),
    SignalDevice2(GameplayConfigSignalDevice),
    SignalBreak(GameplayConfigSignalBreak),
    RenjuChess(GameplayConfigRenjuChess),
    LifePoint(GameplayConfigLifePoint),
    MorseCode(GameplayConfigMorseCode),
    Cipher(GameplayConfigCipher),
    BrokenRock(GameplayConfigBrokenRock),
    FishingRoulette,
    DaolingAuthentication,
    SundialPuzzle,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct OpenSimpleGameplay {
    pub gameplay_config: GameplayConfig,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PlayEffectPos {
    pub r#type: i32, // TODO: Enum?? Type 0 is offset, type 1 is entity id, type 2 is pos
    pub offset: Option<Point>,
    pub entity_id: Option<i64>,
    pub pos: Option<Point>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PlayEffect {
    pub path: String,
    pub pos2: PlayEffectPos,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PlayEffect2 {
    pub r#type: String, // TODO: Convert to enum
    pub path: String,
    pub pos2: Option<PlayEffectPos>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RestorePlayerCameraAdjustment {}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AdjustPlayerCamera {
    // TODO:Implement this
    #[cfg(feature = "strict_json_fields")]
    pub option: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TeammateTeleportConfig {
    pub r#type: i32,
    pub time_out: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TelePortConfig {
    pub r#type: i32, // TODO: Enum??
    pub target_pos: Option<TeleportPosition>,
    pub entity_ids: Option<Vec<i64>>,
    pub transition_option: Option<TransitionOption>,
    pub teammate_teleport_config: Option<TeammateTeleportConfig>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetPlayerPos {
    pub tele_port_config: TelePortConfig,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct MoveTargetEntity {
    pub entity_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum MoveTarget {
    Entity(MoveTargetEntity),
    Player,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct MoveWithSpline {
    pub move_target: MoveTarget,
    pub spline_entity_id: i64,
    pub is_look_dir: Option<bool>,
    pub start_point_index: Option<i32>,
    pub end_point_index: Option<i32>,
    // TODO:Implement this
    #[cfg(feature = "strict_json_fields")]
    pub check_climb: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EnableSplineMoveModel {
    // TODO:Implement this
    #[cfg(feature = "strict_json_fields")]
    pub check_climb: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ToggleScanSplineEffect {
    // TODO:Implement this
    #[cfg(feature = "strict_json_fields")]
    pub check_climb: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct MoveSceneItem {
    pub entity_id: i64,
    // TODO:Implement this
    #[cfg(feature = "strict_json_fields")]
    pub move_config: serde_json::Value,
    pub stop_before_move: Option<bool>,
    pub bypass_client_response: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StopSceneItemMove {
    pub entity_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct FireBullet {
    // TODO:Implement this
    #[cfg(feature = "strict_json_fields")]
    pub r#type: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ClearFishingCabinInSaleItems {}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AcceptFishingEntrust {
    // TODO:Implement this
    #[cfg(feature = "strict_json_fields")]
    pub r#type: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DestroyFishingBoat {
    // TODO:Implement this
    #[cfg(feature = "strict_json_fields")]
    pub r#type: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetJigsawItem {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetJigsawFoundation {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetTeleControl {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetEntityClientVisible {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ToggleHighlightExploreUi {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ExecAlertSystemAction {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AddFlowInteractOption {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RemoveFlowInteractOption {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EnableHostility {
    pub is_enable: bool,
    pub entity_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ChangePhantomFormation {
    pub formation_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RestorePhantomFormation {
    pub retain_phantom: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ChangeTimerType {
    pub r#type: OpType,
    pub time: Option<i32>,
    pub var_for_time: Option<Var>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ChangeTimer {
    pub change_type: ChangeTimerType,
    pub timer_type: TimerType,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ToggleTimerPauseState {
    pub is_pause: bool,
    pub timer_type: TimerType,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ChangeFightTeam {}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AddTrialFollowShooter {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RemoveTrialFollowShooter {
    pub id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AddTrialCharacter {
    pub character_id: i64,
    pub auto_change: Option<bool>,
    pub character_group: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RemoveTrialCharacter {
    pub character_id: i64,
    pub character_group: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetAreaState {
    pub area_id: i32,
    pub state: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SwitchSubLevels {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ChangeTeamPosition {
    pub position_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    pub item_id: i64,
    pub count: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GetItem {
    pub items: Vec<Item>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CreatePrefab {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DestroyPrefab {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PlayDynamicSettlement {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct UsePhantomSkill {
    pub skill_type: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct HideTargetRange {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ChangeOtherState {
    pub entity_id: i64,
    pub state: Flow,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetRegionConfigMpc {
    pub region_mpc_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum SetRegionConfig {
    Mpc(SetRegionConfigMpc),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetReviveRegion {
    pub set_revive_type: SetType,
    pub revive_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ExecResurrection {
    pub revive_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ShowTargetRange {
    pub delay_show: bool,
    pub range_entities: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetTime {
    pub hour: i32,
    pub min: i32,
    pub show_ui: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetTimeLockState {
    pub lock_state: String, // TODO: Enum Lock Unlock
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EnableSystem {
    pub system_type: i32,
    pub is_enable: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EnableAoiNotify {
    pub state: String, // TODO: Enum
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetForceLock {
    pub entity_id: i64,
    pub is_locked: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PlayRegisteredMontage {
    pub entity_id: i64,
    pub montage_id: i64,
    pub is_abp_montage: bool,
    pub repeat_times: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AudioConfig {
    pub group: String,
    pub state: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetAudioState {
    pub audio_config: AudioConfig,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct HideGroup {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ShowHidedGroup {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct HideSpecificEntities {
    pub entity_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ShowSpecificEntities {
    pub entity_ids: Vec<i64>,
    pub delay_show: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RemovePreloadResource {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Preload {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EnableAI {
    pub entity_ids: Vec<i64>,
    pub is_enable: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SwitchDataLayers {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DestroyQuest {
    pub quest_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DestroyQuestItem {
    pub item_id: i64,
    pub count: i32,
    pub is_all: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PromptQuestChapterUI {
    pub chapter_state: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TakePlotPhoto {}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetWuYinQuState {
    pub wu_yin_qu_name: String,
    pub state: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RunActions {
    pub action_list: Vec<Action>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum OccupationType {
    Request,
    Release,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ManualOccupations {
    pub occupation_type: OccupationType,
    pub occupations: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetWeather {
    pub weather_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SendNpcMail {
    pub mail_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EnableFunction {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct FocusOnMapMark {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CharacterLookAt {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AddGuestCharacter {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RemoveGuestCharacter {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TeleportToAndEnterVehicle {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetAreaTimeState {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ResetPlayerCameraFocus {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ResetLevelPlay {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct VehicleSprint {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct VehicleMoveWithPathLine {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ClientPreEnableSubLevels {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GuestOperateUiAnimation {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ChangeEntityCamp {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct NewMoveWithSpline {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DangoAbyssActivatePortal {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DangoAbyssCreateRewardTreasureBox {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DangoAbyssGotoNextFloor {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DangoAbyssReceiveReward {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SummonEntity {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GetRewardByInteract {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct OpenQte {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ActiveAntiGravitySafePoint {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct BvbPlayDialog {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct BvbSendSystemEvent {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct BvbSendAiEvent {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct BvbPlayerOperationConstraint {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ExecClientBattleAction {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TriggerSpecificScanEffect {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SetActorVar {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RunActorCustomEvent {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StopUiScreenEffect {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StopNewMoveWithSpline {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RequestSystemFunction {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Name")]
pub enum Action {
    SetBattleState(ActionFields<SetBattleStateParams>),
    ExecBattleAction(ActionFields<ExecBattleAction>),
    WaitBattleCondition(ActionFields<WaitBattleCondition>),
    PlayFlow(ActionFields<Flow>),
    Collect(ActionFields<CollectParams>),
    LeisureInteract(ActionFields<LeisureInteract>),
    UnlockTeleportTrigger(ActionFields<UnlockTeleportTrigger>),
    EnableTemporaryTeleport(ActionFields<EnableTemporaryTeleport>),
    OpenSystemBoard(ActionFields<OpenSystemBoard>),
    OpenSystemFunction(ActionFields<OpenSystemFunction>),
    ChangeSelfEntityState(ActionFields<ChangeSelfEntityState>),
    SetPlayerOperationRestriction(ActionFields<SetPlayerOperationRestriction>),
    Wait(ActionFields<Wait>),
    ChangeEntityState(ActionFields<ChangeEntityState>),
    Log(ActionFields<Log>),
    EnableNearbyTracking(ActionFields<EnableNearbyTracking>),
    TeleportDungeon(ActionFields<TeleportDungeon>),
    DestroySelf(ActionFields<DestroySelf>),
    CameraLookAt(ActionFields<CameraLookAt>),
    StopCameraLookAt(ActionFields<StopCameraLookAt>),
    EnterOrbitalCamera(ActionFields<EnterOrbitalCamera>),
    ExitOrbitalCamera(ActionFields<ExitOrbitalCamera>),
    SendAiEvent(ActionFields<SendAiEvent>),
    SetInteractionLockState(ActionFields<SetInteractionLockState>),
    AwakeEntity(ActionFields<AwakeEntity>),
    ChangeLiftTarget(ActionFields<ChangeLiftTarget>),
    CalculateVar(ActionFields<CalculateVar>),
    AddBuffToPlayer(ActionFields<AddBuffToPlayer>),
    RemoveBuffFromPlayer(ActionFields<RemoveBuffFromPlayer>),
    AddBuffToEntity(ActionFields<AddBuffToEntity>),
    RemoveBuffFromEntity(ActionFields<RemoveBuffFromEntity>),
    Prompt(ActionFields<Prompt>),
    SetEntityVisible(ActionFields<SetEntityVisible>),
    DestroyEntity(ActionFields<DestroyEntity>),
    GuideTrigger(ActionFields<GuideTypes>),
    TriggerCameraShake(ActionFields<TriggerCameraShake>),
    SetVar(ActionFields<SetVar>),
    VehicleEnter(ActionFields<VehicleEnter>),
    VehicleExitPlayer(ActionFields<VehicleExitPlayer>),
    LockEntity(ActionFields<LockEntity>),
    UnlockEntity(ActionFields<UnlockEntity>),
    CommonTip(ActionFields<CommonTip>),
    CommonTip2(ActionFields<CommonTip2>),
    PostAkEvent(ActionFields<PostAkEvent>),
    VehicleEnterNpc(ActionFields<VehicleEnterNpc>),
    VehicleExitNpc(ActionFields<VehicleExitNpc>),
    PlayerLookAt(ActionFields<PlayerLookAt>),
    PlayBubble(ActionFields<PlayBubble>),
    AddPlayBubble(ActionFields<AddPlayBubble>),
    ClearPlayBubble(ActionFields<ClearPlayBubble>),
    ExecRiskHarvestEffect(ActionFields<ExecRiskHarvestEffect>),
    EnableLevelPlay(ActionFields<EnableLevelPlay>),
    ClaimLevelPlayReward(ActionFields<ClaimLevelPlayReward>),
    SettlementDungeon(ActionFields<SettlementDungeon>),
    ExitDungeon(ActionFields<ExitDungeon>),
    FinishDungeon(ActionFields<FinishDungeon>),
    RecordDungeonEvent(ActionFields<RecordDungeonEvent>),
    RecoverDurability(ActionFields<RecoverDurability>),
    FadeInScreen(ActionFields<FadeInScreen>),
    FadeOutScreen(ActionFields<FadeOutScreen>),
    ChangeNpcPerformState(ActionFields<ChangeNpcPerformState>),
    EntityTurnTo(ActionFields<EntityTurnTo>),
    EntityLookAt(ActionFields<EntityLookAt>),
    ToggleMapMarkState(ActionFields<ToggleMapMarkState>),
    RandomVar(ActionFields<RandomVar>),
    ModifySceneItemAttributeTag(ActionFields<ModifySceneItemAttributeTag>),
    VehicleWaterfallClimbing(ActionFields<VehicleWaterfallClimbing>),
    VehicleTeleport(ActionFields<VehicleTeleport>),
    RogueGotoNextFloor(ActionFields<RogueGotoNextFloor>),
    RogueReceiveReward(ActionFields<RogueReceiveReward>),
    RogueSelectRoom(ActionFields<RogueSelectRoom>),
    RogueActivatePortal(ActionFields<RogueActivatePortal>),
    MowingTowerGotoNextFloor(ActionFields<MowingTowerGotoNextFloor>),
    SlashAndTowerGotoNextFloor(ActionFields<SlashAndTowerGotoNextFloor>),
    PlayMontage(ActionFields<PlayMontage>),
    OpenSystemBoardWithReturn(ActionFields<OpenSystemBoardWithReturn>),
    UnlockSystemItem(ActionFields<UnlockSystemItem>),
    SetSportsState(ActionFields<SetSportsState>),
    OpenSimpleGameplay(ActionFields<OpenSimpleGameplay>),
    PlayEffect(ActionFields<PlayEffect>),
    PlayEffect2(ActionFields<PlayEffect2>),
    RestorePlayerCameraAdjustment(ActionFields<RestorePlayerCameraAdjustment>),
    AdjustPlayerCamera(ActionFields<AdjustPlayerCamera>),
    SetPlayerPos(ActionFields<SetPlayerPos>),
    MoveWithSpline(ActionFields<MoveWithSpline>),
    EnableSplineMoveModel(ActionFields<EnableSplineMoveModel>),
    ToggleScanSplineEffect(ActionFields<ToggleScanSplineEffect>),
    MoveSceneItem(ActionFields<MoveSceneItem>),
    StopSceneItemMove(ActionFields<StopSceneItemMove>),
    FireBullet(ActionFields<FireBullet>),
    ClearFishingCabinInSaleItems(ActionFields<ClearFishingCabinInSaleItems>),
    AcceptFishingEntrust(ActionFields<AcceptFishingEntrust>),
    DestroyFishingBoat(ActionFields<DestroyFishingBoat>),
    SetJigsawItem(ActionFields<SetJigsawItem>),
    SetJigsawFoundation(ActionFields<SetJigsawFoundation>),
    SetTeleControl(ActionFields<SetTeleControl>),
    SetEntityClientVisible(ActionFields<SetEntityClientVisible>),
    ToggleHighlightExploreUi(ActionFields<ToggleHighlightExploreUi>),
    ExecAlertSystemAction(ActionFields<ExecAlertSystemAction>),
    AddFlowInteractOption(ActionFields<AddFlowInteractOption>),
    RemoveFlowInteractOption(ActionFields<RemoveFlowInteractOption>),
    EnableHostility(ActionFields<EnableHostility>),
    ChangePhantomFormation(ActionFields<ChangePhantomFormation>),
    RestorePhantomFormation(ActionFields<RestorePhantomFormation>),
    ChangeTimer(ActionFields<ChangeTimer>),
    ToggleTimerPauseState(ActionFields<ToggleTimerPauseState>),
    ChangeFightTeam(ActionFields<ChangeFightTeam>),
    AddTrialFollowShooter(ActionFields<AddTrialFollowShooter>),
    RemoveTrialFollowShooter(ActionFields<RemoveTrialFollowShooter>),
    AddTrialCharacter(ActionFields<AddTrialCharacter>),
    RemoveTrialCharacter(ActionFields<RemoveTrialCharacter>),
    SetAreaState(ActionFields<SetAreaState>),
    SwitchSubLevels(ActionFields<SwitchSubLevels>),
    ChangeTeamPosition(ActionFields<ChangeTeamPosition>),
    GetItem(ActionFields<GetItem>),
    CreatePrefab(ActionFields<CreatePrefab>),
    DestroyPrefab(ActionFields<DestroyPrefab>),
    CompleteGuide(ActionFields<GuideTypes>),
    PlayDynamicSettlement(ActionFields<PlayDynamicSettlement>),
    UsePhantomSkill(ActionFields<UsePhantomSkill>),
    HideTargetRange(ActionFields<HideTargetRange>),
    ChangeOtherState(ActionFields<ChangeOtherState>),
    SetRegionConfig(ActionFields<SetRegionConfig>),
    SetReviveRegion(ActionFields<SetReviveRegion>),
    ExecResurrection(ActionFields<ExecResurrection>),
    ShowTargetRange(ActionFields<ShowTargetRange>),
    SetTime(ActionFields<SetTime>),
    SetTimeLockState(ActionFields<SetTimeLockState>),
    EnableSystem(ActionFields<EnableSystem>),
    EnableAoiNotify(ActionFields<EnableAoiNotify>),
    SetForceLock(ActionFields<SetForceLock>),
    PlayRegisteredMontage(ActionFields<PlayRegisteredMontage>),
    SetAudioState(ActionFields<SetAudioState>),
    HideGroup(ActionFields<HideGroup>),
    ShowHidedGroup(ActionFields<ShowHidedGroup>),
    HideSpecificEntities(ActionFields<HideSpecificEntities>),
    ShowSpecificEntities(ActionFields<ShowSpecificEntities>),
    RemovePreloadResource(ActionFields<RemovePreloadResource>),
    Preload(ActionFields<Preload>),
    EnableAI(ActionFields<EnableAI>),
    SwitchDataLayers(ActionFields<SwitchDataLayers>),
    DestroyQuest(ActionFields<DestroyQuest>),
    DestroyQuestItem(ActionFields<DestroyQuestItem>),
    PromptQuestChapterUI(ActionFields<PromptQuestChapterUI>),
    TakePlotPhoto(ActionFields<TakePlotPhoto>),
    SetWuYinQuState(ActionFields<SetWuYinQuState>),
    RunActions(ActionFields<RunActions>),
    ManualOccupations(ActionFields<ManualOccupations>),
    SetWeather(ActionFields<SetWeather>),
    SendNpcMail(ActionFields<SendNpcMail>),
    EnableFunction(ActionFields<EnableFunction>),
    FocusOnMapMark(ActionFields<FocusOnMapMark>),
    CharacterLookAt(ActionFields<CharacterLookAt>),
    AddGuestCharacter(ActionFields<AddGuestCharacter>),
    RemoveGuestCharacter(ActionFields<RemoveGuestCharacter>),
    TeleportToAndEnterVehicle(ActionFields<TeleportToAndEnterVehicle>),
    SetAreaTimeState(ActionFields<SetAreaTimeState>),
    ResetPlayerCameraFocus(ActionFields<ResetPlayerCameraFocus>),
    ResetLevelPlay(ActionFields<ResetLevelPlay>),
    VehicleSprint(ActionFields<VehicleSprint>),
    VehicleMoveWithPathLine(ActionFields<VehicleMoveWithPathLine>),
    ClientPreEnableSubLevels(ActionFields<ClientPreEnableSubLevels>),
    GuestOperateUiAnimation(ActionFields<GuestOperateUiAnimation>),
    ChangeEntityCamp(ActionFields<ChangeEntityCamp>),
    NewMoveWithSpline(ActionFields<NewMoveWithSpline>),
    DangoAbyssActivatePortal(ActionFields<DangoAbyssActivatePortal>),
    DangoAbyssCreateRewardTreasureBox(ActionFields<DangoAbyssCreateRewardTreasureBox>),
    DangoAbyssGotoNextFloor(ActionFields<DangoAbyssGotoNextFloor>),
    DangoAbyssReceiveReward(ActionFields<DangoAbyssReceiveReward>),
    SummonEntity(ActionFields<SummonEntity>),
    GetRewardByInteract(ActionFields<GetRewardByInteract>),
    OpenQte(ActionFields<OpenQte>),
    ActiveAntiGravitySafePoint(ActionFields<ActiveAntiGravitySafePoint>),
    BvbPlayDialog(ActionFields<BvbPlayDialog>),
    BvbSendSystemEvent(ActionFields<BvbSendSystemEvent>),
    BvbSendAiEvent(ActionFields<BvbSendAiEvent>),
    BvbPlayerOperationConstraint(ActionFields<BvbPlayerOperationConstraint>),
    ExecClientBattleAction(ActionFields<ExecClientBattleAction>),
    TriggerSpecificScanEffect(ActionFields<TriggerSpecificScanEffect>),
    SetActorVar(ActionFields<SetActorVar>),
    RunActorCustomEvent(ActionFields<RunActorCustomEvent>),
    StopUiScreenEffect(ActionFields<StopUiScreenEffect>),
    StopNewMoveWithSpline(ActionFields<StopNewMoveWithSpline>),
    RequestSystemFunction(ActionFields<RequestSystemFunction>),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Actions {
    pub point: i32,
    pub actions: Vec<Action>,
}