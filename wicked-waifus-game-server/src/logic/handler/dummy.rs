use crate::logic::player::Player;

macro_rules! dummy_handler {
    ($($type_name:ident;)*) => {

        $(::paste::paste! {
            use wicked_waifus_protocol::{[<$type_name Request>], [<$type_name Response>]};
        })*

        $(::paste::paste! {
            pub fn [<on_ $type_name:snake _request>](
                _player: &Player,
                _request: [<$type_name Request>],
                _response: &mut [<$type_name Response>],
            ) {
                tracing::warn!("Unhandled dummy request: {}", stringify!([<$type_name:snake _request>]));
            }
        })*
    };
}

// TODO: implement this
dummy_handler! {
    RoleVisionRecommendData;
    RoleVisionRecommendAttr;
    GetFormationData;
    FishingData;
    EnergySync;
    GetDetectionLabelInfo;
    InfluenceInfo;
    ForgeInfo;
    AchievementInfo;
    ExchangeReward;
    Liveness;
    PhotoMemory;
    VisionEquipGroupInfo;
    UpdatePlayStationBlockAccount;
    AdventureManual;
    Tower;
    ExploreProgress;
    ReportData;
    SimpleTrackReportAsync;
    TowerSeasonUpdate;
    ValidTimeItem;
    PayShopInfo;
    PayInfo;
    InitRange;
    Activity;
    BattlePass;
    SlashAndTowerInfo;
    EntityPatrolStop;
    Advice;
    PlayerTitleData;
    LoadingConfig;
}