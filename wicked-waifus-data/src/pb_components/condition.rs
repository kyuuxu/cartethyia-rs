use serde::Deserialize;

use crate::pb_components::common::{Location, Point};
use crate::pb_components::flow::Flow;
use crate::pb_components::interact::MatchRoleOption;
use crate::pb_components::timer::TimerType;
use crate::pb_components::var::{CompareType, Var};

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareTimePeriod {
    pub compare: CompareType,
    pub time_period: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckChildQuestFinished {
    pub quest_id: i64,
    pub child_quest_id: i64,
    pub compare: Option<CompareType>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareEntityState {
    pub entity_id: i64,
    pub compare: CompareType,
    pub state: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckEntityStateSingle {
    pub entity_id: i64,
    pub state: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckEntityState {
    pub conditions: Vec<CheckEntityStateSingle>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareVarSingle {
    pub var1: Var,
    pub compare: CompareType,
    pub var2: Var,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareVarMultiple {
    pub conditions: Vec<CompareVarSingle>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CompareVar {
    CompareVarSingle(CompareVarSingle),
    CompareVarMultiple(CompareVarMultiple),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareWeather {
    pub compare: CompareType,
    pub weather_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareQuestState {
    pub quest_id: i64,
    pub compare: CompareType,
    pub state: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareEntitySelfState {
    pub compare: CompareType,
    pub state: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckPlayerStateRestriction {
    pub restriction_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct HourMinutes {
    pub hour: i32,
    pub min: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct HourToHour {
    pub compare: CompareType,
    pub start: HourMinutes,
    pub end: HourMinutes,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ComparePlayerMotionState {
    pub compare: CompareType,
    pub motion_state: String, // enum??
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ComparePlayerMotionState2 {
    pub compare: CompareType,
    pub motion_state: String, // enum??
}

#[derive(Deserialize, Debug, Clone)]
pub enum CheckAiStateType {
    AnimalSitDown,
    AnimalStandUp,
    AnimalRandomAction,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckAiState {
    pub compare: CompareType,
    pub state_type: CheckAiStateType,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PreLevelPlay {
    pub pre_level_play: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckLevelPlay {
    pub level_play_ids: Vec<i64>,
    pub complete_number: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckLevelPlayState {
    pub level_id: i32,
    pub compare: CompareType,
    pub state: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckLevelPlayCompleteNumber {
    pub level_id: i32,
    pub compare: CompareType,
    pub number: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareLevelPlayRewardState {
    pub compare: CompareType,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    pub item_id: i32,
    pub count: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckItems {
    pub compare: CompareType,
    pub items: Vec<Item>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct HandInItems {
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
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
pub struct OnlinePlayerConditionTargetOptionTypeParticipator {
    pub any_player: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct OnlinePlayerConditionTargetOptionTypeHost {}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum OnlinePlayerConditionTargetOption {
    Participator(OnlinePlayerConditionTargetOptionTypeParticipator),
    Host(OnlinePlayerConditionTargetOptionTypeHost),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct HasBuff {
    pub buff_id: i64,
    pub compare: CompareType,
    pub online_player_condition_target_option: Option<OnlinePlayerConditionTargetOption>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareLift {
    pub is_self: bool,
    pub location: i32,
    pub compare: CompareType,
    pub entity_id: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckJigsawItemMove {
    pub item_entity_id: i64,
    pub compare: CompareType,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PlaceIndex {
    pub row_index: i32,
    pub column_index: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckJigsawItemPlaceIndex {
    pub item_entity_id: i64,
    pub foundation_entity_id: i64,
    pub place_index: PlaceIndex,
    pub compare: CompareType,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum JigsawCondition {
    CheckJigsawItemMove(CheckJigsawItemMove),
    CheckJigsawItemPlaceIndex(CheckJigsawItemPlaceIndex),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckJigsawInfo {
    pub jigsaw_condition: JigsawCondition,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckInCombat {
    pub in_combat: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DetectCombatState {
    pub entity_id: i64,
    pub state: String,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DetectCombatState2 {
    pub count: i32,
    pub conditions: Vec<DetectCombatState>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct UsingVehicle {
    pub check_is_being_used: bool,
    pub seat: i32,
    pub target_vehicle: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum VehicleType {
    FishingBoat,
    Gongduola,
    SceneItemAutoMoveVehicle,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct PlayerInVehicle {
    pub check_type: bool,
    pub online_player_condition_target_option: Option<OnlinePlayerConditionTargetOption>,
    pub vehicle_type: Option<VehicleType>, // TODO: Review this
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum CheckVehicleConditionType {
    UsingVehicle(UsingVehicle),
    PlayerInVehicle(PlayerInVehicle),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckVehicleCondition {
    pub condition: CheckVehicleConditionType,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckSystemFunction {
    pub system_id: i32,
    pub compare: CompareType,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CollectionShopFull {
    pub shop_type: String, // TODO: Enum
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TrackMoonBuilding {
    pub building_id: i32,
    pub is_built: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TrackMoonPopularity {
    pub compare: CompareType,
    pub popularity: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum CheckSystemStateType {
    CollectionShopFull(CollectionShopFull),
    TrackMoonBuilding(TrackMoonBuilding),
    TrackMoonPopularity(TrackMoonPopularity),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckSystemState {
    pub config: CheckSystemStateType,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckCollectAnimalParts {
    pub check_type: i32,
    pub slots: Vec<i32>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckCurrentRole {
    pub compare: CompareType,
    pub role_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareExploreLevel {
    pub compare: CompareType,
    pub level: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ExploreLevel {
    pub explore_level: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareDungeonId {
    pub compare: CompareType,
    pub dungeon_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DungeonId {
    pub dungeon_id: i32,
    pub count: Option<i32>,
    pub is_exit_dungeon: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DungeonSubType {
    pub dungeon_sub_type: i32,
    pub count: Option<i32>,
    pub is_exit_dungeon: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum DungeonType {
    DungeonId(DungeonId),
    DungeonSubType(DungeonSubType),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct EnterDungeon {
    pub enter_type: DungeonType,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct LeaveDungeon {}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct FinishDungeon {
    pub condition: DungeonType,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckDungeonFinish {
    pub dungeon_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckDungeonHasSaveConfig {
    pub dungeon_id: i32,
    pub is_has_save_config: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareCalabashLevel {
    pub compare: CompareType,
    pub calabash_level: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckCalabashDevelopReward {
    pub monster_id: i64,
    pub develop: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckLordGymFinish {
    pub lord_gym_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GroupConditionElement {
    pub entity_id: i64,
    pub state: String,
    pub is_locked: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct GroupCondition {
    pub conditions: Vec<GroupConditionElement>,
    pub count: i32,
    pub compare: CompareType,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareEntityGroupState {
    pub group_condition: GroupCondition,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckEntityLocked {
    pub entities: Vec<i64>,
    pub is_locked: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckRogueAbilitySelect {
    pub board_id: i32,
    pub is_received: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareFishingBoatState {
    pub is_stop: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompleteCertainFishingEntrust {
    pub id: i32,
    pub count: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareFishingPrestigeLevel {
    pub compare: CompareType,
    pub prestige_level: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckCertainFishingItemCount {
    pub id: i32,
    pub compare: CompareType,
    pub count: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareFishingTechLevel {
    pub tech_id: i32,
    pub compare: CompareType,
    pub tech_level: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ListenEntitySelfEvent {
    pub event_key: String, // TODO: enum??
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckHookLockPoint {
    pub entity_ids: Option<Vec<i64>>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckEntitesExist {
    pub entity_ids: Vec<i64>,
    pub is_exist: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckEntityHasSceneItemAttributeTag {
    pub check_type: i32,
    pub entity_id: i64,
    pub tags: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum AttributeOptionType {
    Health,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AttributeOption {
    pub r#type: AttributeOptionType,
    pub min: i32,
    pub max: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct BattleConditionAttributePlayer {
    pub option: AttributeOption,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct BattleConditionAttributeMonster {
    pub option: AttributeOption,
    pub entity_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct BattleConditionBattleTag {
    pub entity_id: i64,
    pub tag_config_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum BattleCondition {
    Player(BattleConditionAttributePlayer),
    Monster(BattleConditionAttributeMonster),
    BattleTag(BattleConditionBattleTag),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckTargetBattleAttribute {
    pub count: i32,
    pub option: Vec<BattleCondition>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckAlertAreaEnabled {
    pub area_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareAlertValue {
    #[cfg(feature = "strict_json_fields")]
    pub config: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ReachArea {
    pub pos: Point,
    pub range: i32,
    pub range_entity_id: Option<i64>,
    pub entity_id: Option<i64>,
    pub range_entities: Option<Vec<i64>>,
    pub pre_conditions: Option<Conditions>,
    pub match_role_option: Option<Vec<MatchRoleOption>>,
    pub effect_path: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EntityDeadPosition {
    pub entity_id: i64,
    pub var: Var,
    pub is_ignore_rot: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Kill {
    pub exist_targets: Vec<i64>,
    pub targets_to_awake: Vec<i64>,
    pub prefab_var: Option<Var>,
    pub entity_dead_positions: Option<Vec<EntityDeadPosition>>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct DoInteract {
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
pub struct MonsterMergedHpBarSettings {
    pub display_buff_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct MonsterCreator {
    pub monster_creator_entity_ids: Vec<i64>,
    pub prefab_var: Option<Var>,
    pub show_monster_merged_hp_bar: Option<bool>,
    pub tid_monster_group_name: Option<String>,
    pub monster_merged_hp_bar_settings: Option<MonsterMergedHpBarSettings>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CheckBySkillId {
    pub skill_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CheckBySkillGenre {
    pub skill_genre: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct CheckVisionId {
    pub vision_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum UseSkillType {
    CheckBySkillId(CheckBySkillId),
    CheckBySkillGenre(CheckBySkillGenre),
    CheckVisionSummonId(CheckVisionId),
    CheckVisionShowId(CheckVisionId),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct UseSkill {
    pub check: UseSkillType,
    pub is_wait_finish: Option<bool>,
    pub pre_conditions: Option<Conditions>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct GetSkill {
    pub skill_type: SkillType,
    pub skill_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Parkour {
    pub config: String,
    pub spline_entity_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Timer {
    pub time: Option<i32>,
    pub timer_type: TimerType,
    pub var_for_time: Option<Var>,
    // TODO UiConfig
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Guide {
    pub guide_group_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct PlayFlow {
    pub flow: Flow,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct InformationViewCheck {
    pub information_view_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ShowUi {
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
    pub keep_ui_open: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckUiGame {
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StartTime {
    pub hour: i32,
    pub minute: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ScheduleTime {
    pub start_time: StartTime,
}

#[derive(Deserialize, Debug, Clone)]
pub enum SkillType {
    UltimateSkill,
    ESkill,
    VisionSkill,
    NormalSkill,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct SkillOption {
    pub r#type: SkillType,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckPlayerSkillReady {
    pub skill_option: SkillOption,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct WaitTime {
    pub days: i32,
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct TakePhoto {
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ParallaxAlign {
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckDirection {
    pub direction: Point,
    pub angle_interval: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckConditionGroup {
    pub condition: Conditions,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckTreasureBeenClaimed {
    pub entity_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct RangeSphere {
    pub center: Location,
    pub radius: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckInRange {
    pub range_entities: Vec<i64>,
    pub in_range: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub enum KuroGender {
    #[serde(rename = "男")]
    Male,
    #[serde(rename = "女")]
    Female,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckPlayerGender {
    pub gender: KuroGender,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckPlayerInput {
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckFormationRoleInfo {
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct AwakeAndLoadEntity {
    pub entity_ids: Vec<i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum ChessWinner {
    Computer,
    Player,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckChessWinner {
    pub chessboard_id: i64,
    pub winner: ChessWinner,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct WalkingPattern {
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckDataLayer {
    pub data_layer_id: i64,
    pub is_load: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckFinishLoading {
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct VisionSetActivated {
    pub set_type: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct HasVisionGetTargetLevel {
    pub target_level: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum Quest {
    VisionSetActivated(VisionSetActivated),
    HasVisionGetTargetLevel(HasVisionGetTargetLevel),
    DoVisionIdentify,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct VisionSystem {
    pub quest: Quest,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ReadMail {
    pub mail_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct ReceiveTelecom {
    pub telecom_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckActivityState {
    pub activity_id: i32,
    pub compare: CompareType,
    pub activity_state: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckSubLevelState {
    // TODO:
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckEntityGravityDirection {
    // TODO:
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckTeleControlState {
    // TODO:
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckEntityReward {
    // TODO:
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckIsGramophonePlayingMusic {
    // TODO:
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckBVBEvent {
    // TODO:
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct FinishBvbChallenge {
    // TODO:
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareActorVarElement {
    // TODO: ActorRef > PathName(String)
    // pub var1: Var,
    // pub compare: CompareType,
    // pub var2: Var,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CompareActorVar {
    pub conditions: Vec<CompareActorVarElement>,
    pub count: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct CheckDangoCultivationProgress {
    // TODO:
    #[cfg(feature = "strict_json_fields")]
    pub UiType: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "Type")]
pub enum Condition {
    CompareTimePeriod(CompareTimePeriod),
    CheckChildQuestFinished(CheckChildQuestFinished),
    CompareEntityState(CompareEntityState),
    CheckEntityState(CheckEntityState),
    CompareVar(CompareVar),
    CompareWeather(CompareWeather),
    CompareQuestState(CompareQuestState),
    CompareEntitySelfState(CompareEntitySelfState),
    CheckPlayerStateRestriction(CheckPlayerStateRestriction),
    HourToHour(HourToHour),
    ComparePlayerMotionState(ComparePlayerMotionState),
    ComparePlayerMotionState2(ComparePlayerMotionState2),
    CheckAiState(CheckAiState),
    PreLevelPlay(PreLevelPlay),
    CheckLevelPlay(CheckLevelPlay),
    CheckLevelPlayState(CheckLevelPlayState),
    CheckLevelPlayCompleteNumber(CheckLevelPlayCompleteNumber),
    CompareLevelPlayRewardState(CompareLevelPlayRewardState),
    CheckItems(CheckItems),
    HandInItems(HandInItems),
    GetItem(GetItem),
    UseItem(Item),
    HasBuff(HasBuff),
    CompareLift(CompareLift),
    CheckJigsawInfo(CheckJigsawInfo),
    CheckInCombat(CheckInCombat),
    DetectCombatState(DetectCombatState),
    DetectCombatState2(DetectCombatState2),
    CheckVehicleCondition(CheckVehicleCondition),
    CheckSystemFunction(CheckSystemFunction),
    CheckSystemState(CheckSystemState),
    CheckCollectAnimalParts(CheckCollectAnimalParts),
    CheckCurrentRole(CheckCurrentRole),
    CompareExploreLevel(CompareExploreLevel),
    ExploreLevel(ExploreLevel),
    CompareDungeonId(CompareDungeonId),
    EnterDungeon(EnterDungeon),
    LeaveDungeon(LeaveDungeon),
    FinishDungeon(FinishDungeon),
    CheckDungeonFinish(CheckDungeonFinish),
    CheckDungeonHasSaveConfig(CheckDungeonHasSaveConfig),
    CompareCalabashLevel(CompareCalabashLevel),
    CheckCalabashDevelopReward(CheckCalabashDevelopReward),
    CheckLordGymFinish(CheckLordGymFinish),
    CompareEntityGroupState(CompareEntityGroupState),
    CheckEntityLocked(CheckEntityLocked),
    CheckRogueAbilitySelect(CheckRogueAbilitySelect),
    CompareFishingBoatState(CompareFishingBoatState),
    CompleteCertainFishingEntrust(CompleteCertainFishingEntrust),
    CompareFishingPrestigeLevel(CompareFishingPrestigeLevel),
    CheckCertainFishingItemCount(CheckCertainFishingItemCount),
    CompareFishingTechLevel(CompareFishingTechLevel),
    ListenEntitySelfEvent(ListenEntitySelfEvent),
    CheckHookLockPoint(CheckHookLockPoint),
    CheckEntitesExist(CheckEntitesExist),
    CheckEntityHasSceneItemAttributeTag(CheckEntityHasSceneItemAttributeTag),
    CheckTargetBattleAttribute(CheckTargetBattleAttribute),
    CheckAlertAreaEnabled(CheckAlertAreaEnabled),
    CompareAlertValue(CheckAlertAreaEnabled),
    ReachArea(ReachArea),
    Kill(Kill),
    DoInteract(DoInteract),
    MonsterCreator(MonsterCreator),
    UseSkill(UseSkill),
    GetSkill(GetSkill),
    Parkour(Parkour),
    Timer(Timer),
    Guide(Guide),
    PlayFlow(PlayFlow),
    InformationViewCheck(InformationViewCheck),
    ShowUi(ShowUi),
    CheckUiGame(CheckUiGame),
    ScheduleTime(ScheduleTime),
    CheckPlayerSkillReady(CheckPlayerSkillReady),
    WaitTime(WaitTime),
    TakePhoto(TakePhoto),
    ParallaxAlign(ParallaxAlign),
    WaitBattleCondition(CheckTargetBattleAttribute),
    CheckDirection(CheckDirection),
    CheckConditionGroup(CheckConditionGroup),
    CheckTreasureBeenClaimed(CheckTreasureBeenClaimed),
    RangeSphere(RangeSphere),
    CheckInRange(CheckInRange),
    CheckPlayerGender(CheckPlayerGender),
    CheckPlayerInput(CheckPlayerInput),
    CheckFormationRoleInfo(CheckFormationRoleInfo),
    AwakeAndLoadEntity(AwakeAndLoadEntity),
    CheckChessWinner(CheckChessWinner),
    WalkingPattern(WalkingPattern),
    CheckDataLayer(CheckDataLayer),
    CheckFinishLoading(CheckFinishLoading),
    VisionSystem(VisionSystem),
    ReadMail(ReadMail),
    ReceiveTelecom(ReceiveTelecom),
    CheckActivityState(CheckActivityState),
    CheckSubLevelState(CheckSubLevelState),
    CheckEntityGravityDirection(CheckEntityGravityDirection),
    CheckTeleControlState(CheckTeleControlState),
    CheckEntityReward(CheckEntityReward),
    CheckIsGramophonePlayingMusic(CheckIsGramophonePlayingMusic),
    CheckBVBEvent(CheckBVBEvent),
    FinishBvbChallenge(FinishBvbChallenge),
    CompareActorVar(CompareActorVar),
    CheckDangoCultivationProgress(CheckDangoCultivationProgress),
}

#[derive(Deserialize, Debug, Clone)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct Conditions {
    pub r#type: i32,
    pub conditions: Vec<Condition>,
}
