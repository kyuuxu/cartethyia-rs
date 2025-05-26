pub use advice::*;
pub use animal::*;
pub use chat::*;
pub use combat::*;
pub use coop::*;
pub use dummy::*;
pub use entity::*;
pub use friend::*;
pub use gacha::*;
pub use inventory::*;
pub use guide::*;
pub use lord_gym::*;
pub use mail::*;
pub use map::*;
pub use misc::*;
pub use role::*;
pub use scene::*;
use wicked_waifus_protocol::message::Message;
pub use skill::*;
pub use teleport::*;
pub use tutorial::*;
pub use weapon::*;

mod advice;
mod animal;
mod chat;
mod combat;
mod coop;
mod dummy;
mod entity;
mod friend;
mod gacha;
mod guide;
mod inventory;
mod lord_gym;
mod mail;
mod map;
mod misc;
mod role;
mod scene;
mod skill;
mod teleport;
mod tutorial;
mod weapon;

macro_rules! handle_request {
    ($($name:ident $(, $inner_package:ident)?;)*) => {
        fn handle_request(player: &mut super::player::Player, mut msg: Message) {
            use ::wicked_waifus_protocol::{MessageID, Protobuf};

            ::paste::paste! {
                match msg.get_message_id() {
                    $(
                        ::wicked_waifus_protocol::$($inner_package::)?[<$name Request>]::MESSAGE_ID => {
                            let Ok(request) = ::wicked_waifus_protocol::$($inner_package::)?[<$name Request>]::decode(&*msg.remove_payload()) else {
                                tracing::debug!("failed to decode {}, player_id: {}", stringify!($($inner_package::)?[<$name Request>]), player.basic_info.id);
                                return;
                            };

                            tracing::debug!("logic: processing request {}", stringify!($($inner_package::)?[<$name Request>]));

                            let mut response = ::wicked_waifus_protocol::$($inner_package::)?[<$name Response>]::default();
                            [<on_ $($inner_package:snake _)? $name:snake _request>](player, request, &mut response);

                            player.respond(response, msg.get_rpc_id());
                        },
                    )*
                    unhandled => {
                         ::tracing::warn!("can't find handler for request with message_id={unhandled}");
                         let tmp = &*msg.remove_payload();
                         let (name, value) = wicked_waifus_protocol::proto_dumper::get_debug_info(
                             unhandled, tmp,
                         ).unwrap_or_else(|err| ("Error", err.to_string()));
                        tracing::debug!("trying to log unhandled data for message {name} with:\n{value}")
                    }
                }
            }
        }
    };
}

macro_rules! handle_push {
    ($($name:ident $(, $inner_package:ident)?;)*) => {
        fn handle_push(player: &mut super::player::Player, mut msg: Message) {
            use ::wicked_waifus_protocol::{MessageID, Protobuf};

            ::paste::paste! {
                match msg.get_message_id() {
                    $(
                        ::wicked_waifus_protocol::$($inner_package::)?[<$name Push>]::MESSAGE_ID => {
                            let Ok(push) = ::wicked_waifus_protocol::$($inner_package::)?[<$name Push>]::decode(&*msg.remove_payload()) else {
                                tracing::debug!("failed to decode {}, player_id: {}", stringify!($($inner_package::)?[<$name Push>]), player.basic_info.id);
                                return;
                            };

                            tracing::debug!("logic: processing push {}", stringify!($($inner_package::)?[<$name Push>]));

                            [<on_ $($inner_package:snake _)? $name:snake _push>](player, push);
                        },
                    )*
                    unhandled => {
                         ::tracing::warn!("can't find handler for push with message_id={unhandled}");
                         let tmp = &*msg.remove_payload();
                         let (name, value) = wicked_waifus_protocol::proto_dumper::get_debug_info(
                             unhandled, tmp,
                         ).unwrap_or_else(|err| ("Error", err.to_string()));
                        tracing::debug!("trying to log unhandled data for message {name} with:\n{value}")
                    }
                }
            }
        }
    };
}

handle_request! {
    // Advice
    Advice;
    AdviceSet;

    // Animal
    AnimalDie;
    AnimalDrop;
    AnimalDestroy;

    // Chat  (TODO: Review TODOs)
    PrivateChat;
    PrivateChatData;
    PrivateChatHistory;
    PrivateChatOperate;

    // Combat (TODO: Review this on_..., port some from go)
    CombatSendPack, combat_message;
    // CombatMessagePostInfo, combat_message; // TODO: Review this niggerianism, Encrypted shadow data

    // Coop
    LobbyList;

    // Entity (TODO: Review this on_..., port some from go)
    EntityActive;
    EntityOnLanded;
    EntityPosition;
    EntityAccessRange;
    EntityInteract;
    EntityFollowTrack;
    GetRewardTreasureBox;

    // Friend (TODO: Implement them)
    FriendAll;
    // FriendApplySend;
    // FriendRecentlyTeam;
    PlayerBasicInfoGet;

    // Gacha
    Gacha;
    GachaInfo;
    GachaUsePool;

    // Guide
    GuideInfo;
    GuideTrigger;
    GuideFinish;

    // Inventory
    NormalItem;
    WeaponItem;
    PhantomItem;
    ValidTimeItem;
    ItemExchangeInfo;

    // Lord Gym (TODO: Review this on_..., port some from go)
    LordGymInfo;

    // Mail
    MailBindInfo;

    // Map
    DarkCoastDelivery;
    MapCancelTrace;
    MapTrace;
    MapTraceInfo;
    MapUnlockFieldInfo;
    PlayerAccessEffectArea;

    // LevelPlayStateListAsyncRequest // Example: "x9l": [{"inst_id": 902,"level_play_ids": [166700009,157700000]}]

    // Misc (TODO: Review this on_..., port some from go)
    InputSetting;
    InputSettingUpdate;
    LanguageSettingUpdate;
    ServerPlayStationPlayOnlyState;
    LoadingConfig;

    // Player (TODO: Review this on_..., port some from go)
    // ModifySignature;
    // ModifyName;
    // ChangeHeadPhoto;
    PlayerTitleData;

    // Role (TODO: Review this on_..., port some from go)
    RoleShowListUpdate;
    ClientCurrentRoleReport;
    RoleFavorList;
    FormationAttr;
    UpdateFormation;
    UnlockRoleSkinList;
    RoleSkinChange;
    FlySkinWear;
    FlySkinWearAllRole;
    RoleLevelUpView;
    PbUpLevelRole;
    RoleBreakThroughView;

    // Scene (TODO: Review this on_..., port some from go)
    SceneTrace;
    SceneLoadingFinish;
    UpdateSceneDate;
    AccessPathTimeServerConfig;
    PlayerHeadData;

    // Shop (TODO: Review this on_..., port some from go)
    // PayInfo;
    // PayGiftInfo;
    // PayShopItemUpdate;
    PayShopInfo;
    // PayShopUpdate;
    // MonthCard;
    PayInfo;

    // Skill (TODO: Review this on_..., port some from go)
    VisionExploreSkillSet;
    ExploreSkillRouletteSet;
    // RoleActivateSkill;

    // Teleport
    TeleportData;
    TeleportTransfer;
    TeleportFinish;

    // Tower (TODO: Review this on_..., port some from go)
    // TowerSeasonUpdate;
    // Tower;

    // Tutorial
    TutorialInfo;
    TutorialReceive;
    TutorialUnlock;

    // Weapon
    WeaponSkin;
    EquipWeaponSkin;
    SendEquipSkin;
    EquipTakeOn;

    // TODO: Implement all this properly, workaround for game enter
    EntityPatrolStop;
    InitRange;
    Activity;
    BattlePass;
    SlashAndTowerInfo;

    // Role
    RoleVisionRecommendData;
    RoleVisionRecommendAttr;
    PlayerMotion;

    // Formation
    GetFormationData;

    // Misc
    FishingData;
    EnergySync;
    GetDetectionLabelInfo;
    MonthCard;
    InfluenceInfo;
    ForgeInfo;
    AchievementInfo;
    ExchangeReward;
    Liveness;
    WebSign;
    PhotoMemory;
    VisionEquipGroupInfo;
    UpdatePlayStationBlockAccount;
    AdventureManual;
    Tower;
    ExploreProgress;
    ReportData;
    UpdateVoxelEnv;
    SimpleTrackReportAsync;
    TowerSeasonUpdate;
}

handle_push! {
    // Entity
    MovePackage;

    // Misc
    VersionInfo;
}

pub fn handle_logic_message(player: &mut super::player::Player, msg: Message) {
    match msg {
        Message::Request { .. } => handle_request(player, msg),
        Message::Push { .. } => handle_push(player, msg),
        _ => tracing::warn!(
            "handle_logic_message: wrong message type: {}, message_id: {}, player_id: {}",
            msg.get_message_type(),
            msg.get_message_id(),
            player.basic_info.id,
        ),
    }
}
