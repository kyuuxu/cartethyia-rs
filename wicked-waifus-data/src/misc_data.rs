use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::pb_components::action::Action;
use crate::pb_components::condition::Condition;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropValueData {
    pub id: i32,
    pub value: f32,
    pub is_ratio: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VectorData([f32; 3]);

impl VectorData {
    pub fn get_x(&self) -> f32 {
        self.0[0]
    }

    pub fn get_y(&self) -> f32 {
        self.0[1]
    }

    pub fn get_z(&self) -> f32 {
        self.0[2]
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RawVectorData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl RawVectorData {
    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_z(&self) -> f32 {
        self.z
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntranceEntityData {
    pub dungeon_id: i32,
    pub entrance_entity_id: i64,
}

#[derive(Deserialize_repr, PartialEq, Debug, Copy, Clone)]
#[repr(i32)]
pub enum GachaViewTypeInfoId {
    NoviceConvene = 1,
    FeaturedResonatorConvene = 2,
    FeaturedWeaponConvene = 3,
    StandardResonatorConvene = 4,
    StandardWeaponConvene = 5,
    BeginnersChoiceConvene = 6,
    MultipleChoiceResonatorConvene = 7,
    MultipleChoiceWeaponConvene = 8,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ConsumeItem {
    pub item_id: i32,
    pub count: i32,
}

#[derive(Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum EntityType {
    AdsorptionFoundation,
    AdviseItem,
    AiAlertNotifier,
    AiGearController,
    AiMovementGear,
    AirPassage,
    AiSceneItem,
    Animal,
    Animal2,
    AnnunciatorCenter,
    AnnunciatorWire,
    AreaOccupation,
    Audio,
    AudioBox,
    BatchBulletCaster,
    BeamCastBullet,
    BeamCaster,
    BeamCrystal,
    BeamDeliver,
    BeamReceiver,
    BondageTrap,
    BuffConsumer,
    BuffProducer,
    BurstCrystalFoundation,
    Chair,
    Chair2,
    ChallengeInteract,
    Chessboard,
    Chessman,
    ClientTrigger,
    Collect,
    Collect2,
    Collect3,
    CollectAnimal,
    CollectAnimalPart,
    CombatAnimal,
    CombatAnimal2,
    CombinedVisibleGroup,
    ControlConnector,
    ConveyorBelt,
    CookTool,
    CurveControlDestructible,
    CustomAoiEditor,
    Destructible,
    DestructibleControl,
    DestructibleExploreInteractor,
    DestructibleSceneBullet,
    DestructibleTrigger,
    Disc,
    Drop,
    DungeonEntry,
    DynamicPortalCreater,
    EffectArea,
    EnrichmentArea,
    EntityBatchRefresh,
    EntityBundle,
    EntityList,
    EntityPackage,
    ExploreSkillInteractor,
    FishingBoat,
    FollowShooter,
    FollowShooter2,
    FollowTrack,
    FollowTrackFoundation,
    Gramophone,
    GravityFlip,
    GroupAi,
    HackingTypeFollowShooter,
    HookLockPoint,
    HookSoundBox,
    HookWithRange,
    HorseBettingTuanzi,
    InhaledItem,
    InteractFoundation,
    InteractFoundationWithSceneItemAttribute,
    InteractGear,
    InteractGearGroup,
    InteractiveConditionListener,
    Item,
    ItemFoundation,
    JigsawFoundation,
    JigsawItem,
    KiteHook,
    LevelPlay,
    LevelPlayReward,
    LevelQteTrigger,
    LevelSeqTrigger,
    LevitateMagnet,
    LifePointCenter,
    Lift,
    LightDeliver,
    LocationSafety,
    Monitor,
    Monster,
    MonsterGachaBase,
    MonsterGachaItem,
    MoveableTrigger,
    NoRenderPortal,
    Npc,
    Npc2,
    PasserbyNpc,
    PhotoTarget,
    PhysicsSwing,
    Portal,
    Position,
    ProgressBarController,
    ProgressBarControllerWithAttribute,
    PullingObject,
    Range,
    RangeTriggerTargetGear,
    ReboundPlateGear,
    RefreshGroup,
    RenderSpecifiedRange,
    RenderSpecifiedRange2,
    Resurrection,
    RewardNpc,
    RollingFireball,
    Rotator,
    SceneAura,
    SceneBullet,
    SceneBulletCanHit,
    SceneBulletWithMovement,
    SceneItemStateHint,
    SimpleInteract,
    SimpleNPc,
    SkyboxTrigger,
    SlideRail,
    SoundBox,
    SpawnMonster,
    SpawnPasserbyNpc,
    Spline,
    SplineRange,
    SummonGongduolaPoint,
    StateSceneItem,
    StateTrigger,
    StatueFoundation,
    SuiGuangHook,
    TargetGear,
    TargetGear2,
    TargetGearGroup,
    TargetGearGroup2,
    TargetGearPro,
    TargetGearWithLevelPrefabPerform,
    TeleControl,
    TeleControl3,
    TeleControlGroup,
    Teleporter,
    TemplateEntitySpawner,
    TemporaryTeleporter,
    TimedStrikeDevice,
    TimelineTrackController,
    TimeStop,
    Trample,
    Trample2,
    Trample3,
    TreasureBox,
    Trigger,
    TriggerConditionListener,
    TuanziNpc,
    TurntableController,
    VacuumCleaner,
    VarManager,
    Vehicle,
    Vehicle2,
    VehicleNpc,
    VehicleSceneItem,
    VisibleTrigger,
    Vision,
    VisionItem,
    VisionTreasureBox,
    WalkingPatternController,
    WaterCollection,
    WaterSpout,
    Weapon,
    WindSource,
}

#[derive(Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum EntityLogic {
    Item,
    Animal,
    Monster,
    Vehicle,
    Npc,
    Vision,
    ClientOnly,
    ServerOnly,
    Custom,
}

#[derive(Debug, Deserialize)]
pub enum LevelPlayType {
    Challenge,
    SilentArea,
    Dungeon,
    MonsterTreasure,
    Quest,
    Riddle,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayOpenCondition {
    pub conditions: Option<Vec<Condition>>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayActive {
    pub active_type: i32,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayRewardConfigResetTypeMidNight {
    pub count: i32,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "Type")]
pub enum LevelPlayRewardConfigResetType {
    MidNight(LevelPlayRewardConfigResetTypeMidNight),
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayRewardConfigInteract {
    pub reward_id: i32,
    pub reward_entity_id: i64,
    pub reward_complete_actions: Vec<Action>,
    pub first_complete_actions: Option<Vec<Action>>,
    pub reset: Option<LevelPlayRewardConfigResetType>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "Type")]
pub enum LevelPlayRewardConfig {
    None,
    Interact(LevelPlayRewardConfigInteract),
}

#[derive(Debug, Deserialize)]
pub enum FixedDateTime {
    Daily,
    Weekly
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayRefreshConfigFixedDateTime {
    pub update_type: FixedDateTime,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayRefreshConfigCompleted {
    pub min_refresh_cd: i32,
    pub max_refresh_cd: i32,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "Type")]
pub enum LevelPlayRefreshConfig {
    None,
    FixedDateTime(LevelPlayRefreshConfigFixedDateTime),
    Completed(LevelPlayRefreshConfigCompleted),
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayTrack {
    pub track_radius: i32,
    pub track_priority: i32,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayMark {
    pub mark_id: i32,
    pub map_bg_path: String,
}

#[derive(Debug, Deserialize)]
pub enum OnlineType {
    Multiplayer,
    Local,
    Hang,
}

#[derive(Debug, Deserialize)]
pub enum ObjType {
    LevelPlay,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct LevelPlayDataDetail { // Json file contains Data in name, so it has to be DataData
    pub id: i32,
    pub key: String,
    #[cfg(feature = "strict_json_fields")]
    pub internal_dest: String,
    pub level_id: i32,
    #[cfg(feature = "strict_json_fields")]
    pub tid_name: String,
    pub r#type: LevelPlayType,
    pub instance_id: i32,
    pub level_play_entity_id: i64,
    pub level_additive_id: i32,
    pub enter_radius: i32,
    pub leave_radius: i32,
    pub delay_refresh: bool,
    pub delay_destroy: bool,
    pub level_play_open_condition: LevelPlayOpenCondition,
    pub level_play_active: LevelPlayActive,
    pub level_play_reward_config: LevelPlayRewardConfig,
    pub level_play_refresh_config: LevelPlayRefreshConfig,
    pub level_play_track: LevelPlayTrack,
    pub level_play_mark: Option<LevelPlayMark>,
    pub enter_in_range_actions: Option<Vec<Action>>,
    pub pack_id: i32,
    pub online_type: OnlineType,
    pub obj_type: ObjType,
    #[cfg(feature = "strict_json_fields")]
    pub children: Option<Vec<String>>,
    #[cfg(feature = "strict_json_fields")]
    pub reference: Vec<String>,
    #[cfg(feature = "strict_json_fields")]
    pub weak_reference: Option<Vec<String>>,
    pub exploratory_degree: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StateMachineTransition {
    pub from: i32,
    pub to: i32,
    pub transition_prediction_type: i32,
    pub weight: i32,
    #[cfg(feature = "strict_json_fields")]
    pub conditions: Vec<serde_json::Value>, // TODO: Implement conditions
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StateMachineNodeCommon {
    pub uuid: i32,
    pub is_anim_state_machine: Option<bool>,
    #[cfg(feature = "strict_json_fields")]
    pub is_conduit_node: Option<bool>,
    #[cfg(feature = "strict_json_fields")]
    pub is_any_state: Option<bool>,
    #[cfg(feature = "strict_json_fields")]
    pub name: String,
    #[cfg(feature = "strict_json_fields")]
    pub take_control_type: i32,
    #[cfg(feature = "strict_json_fields")]
    pub transition_rule: i32,
    pub children: Option<Vec<i32>>,
    pub transitions: Option<Vec<StateMachineTransition>>,
    #[cfg(feature = "strict_json_fields")]
    pub on_enter_actions: Option<Vec<serde_json::Value>>,  // TODO: Implement actions
    #[cfg(feature = "strict_json_fields")]
    pub on_exit_actions: Option<Vec<serde_json::Value>>,  // TODO: Implement actions
    #[cfg(feature = "strict_json_fields")]
    pub bind_states: Option<Vec<serde_json::Value>>,  // TODO: Implement bindStates
    #[cfg(feature = "strict_json_fields")]
    pub task: Option<serde_json::Value>,  // TODO: Implement bindStates
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StateMachineNodeReferenced {
    pub reference_uuid: i32,
    #[serde(flatten)]
    pub common: StateMachineNodeCommon,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StateMachineNodeOverrideCommon {
    pub override_common_uuid: i32,
    #[serde(flatten)]
    pub common: StateMachineNodeCommon,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StateMachineNodeCustom {
    #[serde(flatten)]
    pub common: StateMachineNodeCommon,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum StateMachineNode {
    Reference(StateMachineNodeReferenced),
    Override(StateMachineNodeOverrideCommon),
    Custom(StateMachineNodeCustom),
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "strict_json_fields", serde(deny_unknown_fields))]
#[serde(rename_all = "PascalCase")]
pub struct StateMachineJson {
    pub version: u32,
    pub state_machines: Vec<i32>,
    pub nodes: Vec<StateMachineNode>,
}