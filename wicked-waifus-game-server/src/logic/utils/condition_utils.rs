use wicked_waifus_data::pb_components::condition::{CheckConditionGroup, CompareEntityState, Condition};

use crate::logic::ecs::component::ComponentContainer;
use crate::logic::player::Player;
use crate::logic::utils::tag_utils;
use crate::query_components;

macro_rules! unimplemented_condition {
    ($condition:ident) => {
        {
            tracing::warn!("Condition check not implemented for: {:?}", $condition);
            true
        }
    }
}

pub fn check_condition(player: &Player,
                       entity_id: i64,
                       level_entity_data: &wicked_waifus_data::LevelEntityConfigData,
                       template_config: &wicked_waifus_data::TemplateConfigData,
                       element: Condition) -> bool {
    match element {
        Condition::CompareTimePeriod(condition) => unimplemented_condition! { condition },
        Condition::CheckChildQuestFinished(condition) => unimplemented_condition! { condition },
        Condition::CompareEntityState(condition) => compare_entity_state(player, entity_id, condition),
        Condition::CheckEntityState(condition) => unimplemented_condition! { condition },
        Condition::CompareVar(condition) => unimplemented_condition! { condition },
        Condition::CompareWeather(condition) => unimplemented_condition! { condition },
        Condition::CompareQuestState(condition) => unimplemented_condition! { condition },
        Condition::CompareEntitySelfState(condition) => unimplemented_condition! { condition },
        Condition::CheckPlayerStateRestriction(condition) => unimplemented_condition! { condition },
        Condition::HourToHour(condition) => unimplemented_condition! { condition },
        Condition::ComparePlayerMotionState(condition) => unimplemented_condition! { condition },
        Condition::ComparePlayerMotionState2(condition) => unimplemented_condition! { condition },
        Condition::CheckAiState(condition) => unimplemented_condition! { condition },
        Condition::PreLevelPlay(condition) => unimplemented_condition! { condition },
        Condition::CheckLevelPlay(condition) => unimplemented_condition! { condition },
        Condition::CheckLevelPlayState(condition) => unimplemented_condition! { condition },
        Condition::CheckLevelPlayCompleteNumber(condition) => unimplemented_condition! { condition },
        Condition::CompareLevelPlayRewardState(condition) => unimplemented_condition! { condition },
        Condition::CheckItems(condition) => unimplemented_condition! { condition },
        Condition::HandInItems(condition) => unimplemented_condition! { condition },
        Condition::GetItem(condition) => unimplemented_condition! { condition },
        Condition::UseItem(condition) => unimplemented_condition! { condition },
        Condition::HasBuff(condition) => unimplemented_condition! { condition },
        Condition::CompareLift(condition) => unimplemented_condition! { condition },
        Condition::CheckJigsawInfo(condition) => unimplemented_condition! { condition },
        Condition::CheckInCombat(condition) => unimplemented_condition! { condition },
        Condition::DetectCombatState(condition) => unimplemented_condition! { condition },
        Condition::DetectCombatState2(condition) => unimplemented_condition! { condition },
        Condition::CheckVehicleCondition(condition) => unimplemented_condition! { condition },
        Condition::CheckSystemFunction(condition) => unimplemented_condition! { condition },
        Condition::CheckSystemState(condition) => unimplemented_condition! { condition },
        Condition::CheckCollectAnimalParts(condition) => unimplemented_condition! { condition },
        Condition::CheckCurrentRole(condition) => unimplemented_condition! { condition },
        Condition::CompareExploreLevel(condition) => unimplemented_condition! { condition },
        Condition::ExploreLevel(condition) => unimplemented_condition! { condition },
        Condition::CompareDungeonId(condition) => unimplemented_condition! { condition },
        Condition::EnterDungeon(condition) => unimplemented_condition! { condition },
        Condition::LeaveDungeon(condition) => unimplemented_condition! { condition },
        Condition::FinishDungeon(condition) => unimplemented_condition! { condition },
        Condition::CheckDungeonFinish(condition) => unimplemented_condition! { condition },
        Condition::CheckDungeonHasSaveConfig(condition) => unimplemented_condition! { condition },
        Condition::CompareCalabashLevel(condition) => unimplemented_condition! { condition },
        Condition::CheckCalabashDevelopReward(condition) => unimplemented_condition! { condition },
        Condition::CheckLordGymFinish(condition) => unimplemented_condition! { condition },
        Condition::CompareEntityGroupState(condition) => unimplemented_condition! { condition },
        Condition::CheckEntityLocked(condition) => unimplemented_condition! { condition },
        Condition::CheckRogueAbilitySelect(condition) => unimplemented_condition! { condition },
        Condition::CompareFishingBoatState(condition) => unimplemented_condition! { condition },
        Condition::CompleteCertainFishingEntrust(condition) => unimplemented_condition! { condition },
        Condition::CompareFishingPrestigeLevel(condition) => unimplemented_condition! { condition },
        Condition::CheckCertainFishingItemCount(condition) => unimplemented_condition! { condition },
        Condition::CompareFishingTechLevel(condition) => unimplemented_condition! { condition },
        Condition::ListenEntitySelfEvent(condition) => unimplemented_condition! { condition },
        Condition::CheckHookLockPoint(condition) => unimplemented_condition! { condition },
        Condition::CheckEntitesExist(condition) => unimplemented_condition! { condition },
        Condition::CheckEntityHasSceneItemAttributeTag(condition) => unimplemented_condition! { condition },
        Condition::CheckTargetBattleAttribute(condition) => unimplemented_condition! { condition },
        Condition::CheckAlertAreaEnabled(condition) => unimplemented_condition! { condition },
        Condition::CompareAlertValue(condition) => unimplemented_condition! { condition },
        Condition::ReachArea(condition) => unimplemented_condition! { condition },
        Condition::Kill(condition) => unimplemented_condition! { condition },
        Condition::DoInteract(condition) => unimplemented_condition! { condition },
        Condition::MonsterCreator(condition) => unimplemented_condition! { condition },
        Condition::UseSkill(condition) => unimplemented_condition! { condition },
        Condition::GetSkill(condition) => unimplemented_condition! { condition },
        Condition::Parkour(condition) => unimplemented_condition! { condition },
        Condition::Timer(condition) => unimplemented_condition! { condition },
        Condition::Guide(condition) => unimplemented_condition! { condition },
        Condition::PlayFlow(condition) => unimplemented_condition! { condition },
        Condition::InformationViewCheck(condition) => unimplemented_condition! { condition },
        Condition::ShowUi(condition) => unimplemented_condition! { condition },
        Condition::CheckUiGame(condition) => unimplemented_condition! { condition },
        Condition::ScheduleTime(condition) => unimplemented_condition! { condition },
        Condition::CheckPlayerSkillReady(condition) => unimplemented_condition! { condition },
        Condition::WaitTime(condition) => unimplemented_condition! { condition },
        Condition::TakePhoto(condition) => unimplemented_condition! { condition },
        Condition::ParallaxAlign(condition) => unimplemented_condition! { condition },
        Condition::WaitBattleCondition(condition) => unimplemented_condition! { condition },
        Condition::CheckDirection(condition) => unimplemented_condition! { condition },
        Condition::CheckConditionGroup(condition) => check_condition_group(player, entity_id, level_entity_data, template_config, condition),
        Condition::CheckTreasureBeenClaimed(condition) => unimplemented_condition! { condition },
        Condition::RangeSphere(condition) => unimplemented_condition! { condition },
        Condition::CheckInRange(condition) => unimplemented_condition! { condition },
        Condition::CheckPlayerGender(condition) => unimplemented_condition! { condition },
        Condition::CheckPlayerInput(condition) => unimplemented_condition! { condition },
        Condition::CheckFormationRoleInfo(condition) => unimplemented_condition! { condition },
        Condition::AwakeAndLoadEntity(condition) => unimplemented_condition! { condition },
        Condition::CheckChessWinner(condition) => unimplemented_condition! { condition },
        Condition::WalkingPattern(condition) => unimplemented_condition! { condition },
        Condition::CheckDataLayer(condition) => unimplemented_condition! { condition },
        Condition::CheckFinishLoading(condition) => unimplemented_condition! { condition },
        Condition::VisionSystem(condition) => unimplemented_condition! { condition },
        Condition::ReadMail(condition) => unimplemented_condition! { condition },
        Condition::ReceiveTelecom(condition) => unimplemented_condition! { condition },
        Condition::CheckActivityState(condition) => unimplemented_condition! { condition },
        Condition::CheckSubLevelState(condition) => unimplemented_condition! { condition },
        Condition::CheckEntityGravityDirection(condition) => unimplemented_condition! { condition },
        Condition::CheckTeleControlState(condition) => unimplemented_condition! { condition },
        Condition::CheckEntityReward(condition) => unimplemented_condition! { condition },
        Condition::CheckIsGramophonePlayingMusic(condition) => unimplemented_condition! { condition },
        Condition::CheckBVBEvent(condition) => unimplemented_condition! { condition },
        Condition::FinishBvbChallenge(condition) => unimplemented_condition! { condition },
        Condition::CompareActorVar(condition) => unimplemented_condition! { condition },
        Condition::CheckDangoCultivationProgress(condition) => unimplemented_condition! { condition },
    }
}

fn compare_entity_state(player: &Player, entity_id: i64, condition: CompareEntityState) -> bool {
    let actual = {
        let world_ref = player.world.borrow();
        let world = world_ref.get_world_entity();
        let state_tag = query_components!(world, entity_id, StateTag).0.unwrap();
        state_tag.state_tag_id
    };
    let expected = tag_utils::get_tag_id_by_name(condition.state.as_str());
    // In theory, we can only check for equal or not equal
    tracing::debug!("CompareEntityState: type {:?}, actual: {actual}, expected: {expected}", condition.compare);
    condition.compare.cmp(&expected, &actual)
}

fn check_condition_group(player: &Player,
                         entity_id: i64,
                         level_entity_data: &wicked_waifus_data::LevelEntityConfigData,
                         template_config: &wicked_waifus_data::TemplateConfigData,
                         condition: CheckConditionGroup) -> bool {

    let mut check = true;
    // TODO: Investigate if type has a meaning
    for element in condition.condition.conditions {
        check = check_condition(player, entity_id, level_entity_data, template_config, element);
        if !check {
            break;
        }
    }
    check
}