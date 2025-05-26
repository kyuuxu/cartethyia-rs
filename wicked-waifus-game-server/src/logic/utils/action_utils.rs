use std::collections::HashMap;

use wicked_waifus_protocol::{CommonTagData, EntityCommonTagNotify, EntityStateReadyNotify, ItemRewardNotify, NormalItemUpdateNotify, RewardItemInfo, WR};

use wicked_waifus_data::pb_components::action::{Action, ChangeSelfEntityState, UnlockTeleportTrigger};
use wicked_waifus_data::pb_components::entity_state::EntityStateComponent;

use crate::logic::ecs::component::ComponentContainer;
use crate::logic::player::{ItemUsage, Player};
use crate::logic::utils::tag_utils;
use crate::query_components;

macro_rules! unimplemented_action {
    ($action:ident) => {
        {
            tracing::warn!("Action not implemented for: {:?}", $action);
        }
    }
}

pub fn perform_action(player: &mut Player,
                      entity_id: i64,
                      level_entity_data: &wicked_waifus_data::LevelEntityConfigData,
                      template_config: &wicked_waifus_data::TemplateConfigData,
                      element: Action) {
    match element {
        Action::SetBattleState(action) => unimplemented_action! { action },
        Action::ExecBattleAction(action) => unimplemented_action! { action },
        Action::WaitBattleCondition(action) => unimplemented_action! { action },
        Action::PlayFlow(action) => unimplemented_action! { action },
        Action::Collect(_) => collect_action(player, level_entity_data, template_config),
        Action::LeisureInteract(action) => unimplemented_action! { action },
        Action::UnlockTeleportTrigger(action) => unlock_teleport_trigger(player, action.params),
        Action::EnableTemporaryTeleport(action) => unimplemented_action! { action },
        Action::OpenSystemBoard(action) => unimplemented_action! { action },
        Action::OpenSystemFunction(action) => unimplemented_action! { action },
        Action::ChangeSelfEntityState(action) => change_self_entity_state(player, entity_id, level_entity_data, template_config, action.params),
        Action::SetPlayerOperationRestriction(action) => unimplemented_action! { action },
        Action::Wait(action) => unimplemented_action! { action },
        Action::ChangeEntityState(action) => unimplemented_action! { action },
        Action::Log(action) => unimplemented_action! { action },
        Action::EnableNearbyTracking(action) => unimplemented_action! { action },
        Action::TeleportDungeon(action) => unimplemented_action! { action },
        Action::DestroySelf(action) => unimplemented_action! { action },
        Action::CameraLookAt(action) => unimplemented_action! { action },
        Action::StopCameraLookAt(action) => unimplemented_action! { action },
        Action::EnterOrbitalCamera(action) => unimplemented_action! { action },
        Action::ExitOrbitalCamera(action) => unimplemented_action! { action },
        Action::SendAiEvent(action) => unimplemented_action! { action },
        Action::SetInteractionLockState(action) => unimplemented_action! { action },
        Action::AwakeEntity(action) => unimplemented_action! { action },
        Action::ChangeLiftTarget(action) => unimplemented_action! { action },
        Action::CalculateVar(action) => unimplemented_action! { action },
        Action::AddBuffToPlayer(action) => unimplemented_action! { action },
        Action::RemoveBuffFromPlayer(action) => unimplemented_action! { action },
        Action::AddBuffToEntity(action) => unimplemented_action! { action },
        Action::RemoveBuffFromEntity(action) => unimplemented_action! { action },
        Action::Prompt(action) => unimplemented_action! { action },
        Action::SetEntityVisible(action) => unimplemented_action! { action },
        Action::DestroyEntity(action) => unimplemented_action! { action },
        Action::GuideTrigger(action) => unimplemented_action! { action },
        Action::TriggerCameraShake(action) => unimplemented_action! { action },
        Action::SetVar(action) => unimplemented_action! { action },
        Action::VehicleEnter(action) => unimplemented_action! { action },
        Action::VehicleExitPlayer(action) => unimplemented_action! { action },
        Action::LockEntity(action) => unimplemented_action! { action },
        Action::UnlockEntity(action) => unimplemented_action! { action },
        Action::CommonTip(action) => unimplemented_action! { action },
        Action::CommonTip2(action) => unimplemented_action! { action },
        Action::PostAkEvent(action) => unimplemented_action! { action },
        Action::VehicleEnterNpc(action) => unimplemented_action! { action },
        Action::VehicleExitNpc(action) => unimplemented_action! { action },
        Action::PlayerLookAt(action) => unimplemented_action! { action },
        Action::PlayBubble(action) => unimplemented_action! { action },
        Action::AddPlayBubble(action) => unimplemented_action! { action },
        Action::ClearPlayBubble(action) => unimplemented_action! { action },
        Action::ExecRiskHarvestEffect(action) => unimplemented_action! { action },
        Action::EnableLevelPlay(action) => unimplemented_action! { action },
        Action::ClaimLevelPlayReward(action) => unimplemented_action! { action },
        Action::SettlementDungeon(action) => unimplemented_action! { action },
        Action::ExitDungeon(action) => unimplemented_action! { action },
        Action::FinishDungeon(action) => unimplemented_action! { action },
        Action::RecordDungeonEvent(action) => unimplemented_action! { action },
        Action::RecoverDurability(action) => unimplemented_action! { action },
        Action::FadeInScreen(action) => unimplemented_action! { action },
        Action::FadeOutScreen(action) => unimplemented_action! { action },
        Action::ChangeNpcPerformState(action) => unimplemented_action! { action },
        Action::EntityTurnTo(action) => unimplemented_action! { action },
        Action::EntityLookAt(action) => unimplemented_action! { action },
        Action::ToggleMapMarkState(action) => unimplemented_action! { action },
        Action::RandomVar(action) => unimplemented_action! { action },
        Action::ModifySceneItemAttributeTag(action) => unimplemented_action! { action },
        Action::VehicleWaterfallClimbing(action) => unimplemented_action! { action },
        Action::VehicleTeleport(action) => unimplemented_action! { action },
        Action::RogueGotoNextFloor(action) => unimplemented_action! { action },
        Action::RogueReceiveReward(action) => unimplemented_action! { action },
        Action::RogueSelectRoom(action) => unimplemented_action! { action },
        Action::RogueActivatePortal(action) => unimplemented_action! { action },
        Action::MowingTowerGotoNextFloor(action) => unimplemented_action! { action },
        Action::SlashAndTowerGotoNextFloor(action) => unimplemented_action! { action },
        Action::PlayMontage(action) => unimplemented_action! { action },
        Action::OpenSystemBoardWithReturn(action) => unimplemented_action! { action },
        Action::UnlockSystemItem(action) => unimplemented_action! { action },
        Action::SetSportsState(action) => unimplemented_action! { action },
        Action::OpenSimpleGameplay(action) => unimplemented_action! { action },
        Action::PlayEffect(action) => unimplemented_action! { action },
        Action::PlayEffect2(action) => unimplemented_action! { action },
        Action::RestorePlayerCameraAdjustment(action) => unimplemented_action! { action },
        Action::AdjustPlayerCamera(action) => unimplemented_action! { action },
        Action::SetPlayerPos(action) => unimplemented_action! { action },
        Action::MoveWithSpline(action) => unimplemented_action! { action },
        Action::EnableSplineMoveModel(action) => unimplemented_action! { action },
        Action::ToggleScanSplineEffect(action) => unimplemented_action! { action },
        Action::MoveSceneItem(action) => unimplemented_action! { action },
        Action::StopSceneItemMove(action) => unimplemented_action! { action },
        Action::FireBullet(action) => unimplemented_action! { action },
        Action::ClearFishingCabinInSaleItems(action) => unimplemented_action! { action },
        Action::AcceptFishingEntrust(action) => unimplemented_action! { action },
        Action::DestroyFishingBoat(action) => unimplemented_action! { action },
        Action::SetJigsawItem(action) => unimplemented_action! { action },
        Action::SetJigsawFoundation(action) => unimplemented_action! { action },
        Action::SetTeleControl(action) => unimplemented_action! { action },
        Action::SetEntityClientVisible(action) => unimplemented_action! { action },
        Action::ToggleHighlightExploreUi(action) => unimplemented_action! { action },
        Action::ExecAlertSystemAction(action) => unimplemented_action! { action },
        Action::AddFlowInteractOption(action) => unimplemented_action! { action },
        Action::RemoveFlowInteractOption(action) => unimplemented_action! { action },
        Action::EnableHostility(action) => unimplemented_action! { action },
        Action::ChangePhantomFormation(action) => unimplemented_action! { action },
        Action::RestorePhantomFormation(action) => unimplemented_action! { action },
        Action::ChangeTimer(action) => unimplemented_action! { action },
        Action::ToggleTimerPauseState(action) => unimplemented_action! { action },
        Action::ChangeFightTeam(action) => unimplemented_action! { action },
        Action::AddTrialFollowShooter(action) => unimplemented_action! { action },
        Action::RemoveTrialFollowShooter(action) => unimplemented_action! { action },
        Action::AddTrialCharacter(action) => unimplemented_action! { action },
        Action::RemoveTrialCharacter(action) => unimplemented_action! { action },
        Action::SetAreaState(action) => unimplemented_action! { action },
        Action::SwitchSubLevels(action) => unimplemented_action! { action },
        Action::ChangeTeamPosition(action) => unimplemented_action! { action },
        Action::GetItem(action) => unimplemented_action! { action },
        Action::CreatePrefab(action) => unimplemented_action! { action },
        Action::DestroyPrefab(action) => unimplemented_action! { action },
        Action::CompleteGuide(action) => unimplemented_action! { action },
        Action::PlayDynamicSettlement(action) => unimplemented_action! { action },
        Action::UsePhantomSkill(action) => unimplemented_action! { action },
        Action::HideTargetRange(action) => unimplemented_action! { action },
        Action::ChangeOtherState(action) => unimplemented_action! { action },
        Action::SetRegionConfig(action) => unimplemented_action! { action },
        Action::SetReviveRegion(action) => unimplemented_action! { action },
        Action::ExecResurrection(action) => unimplemented_action! { action },
        Action::ShowTargetRange(action) => unimplemented_action! { action },
        Action::SetTime(action) => unimplemented_action! { action },
        Action::SetTimeLockState(action) => unimplemented_action! { action },
        Action::EnableSystem(action) => unimplemented_action! { action },
        Action::EnableAoiNotify(action) => unimplemented_action! { action },
        Action::SetForceLock(action) => unimplemented_action! { action },
        Action::PlayRegisteredMontage(action) => unimplemented_action! { action },
        Action::SetAudioState(action) => unimplemented_action! { action },
        Action::HideGroup(action) => unimplemented_action! { action },
        Action::ShowHidedGroup(action) => unimplemented_action! { action },
        Action::HideSpecificEntities(action) => unimplemented_action! { action },
        Action::ShowSpecificEntities(action) => unimplemented_action! { action },
        Action::RemovePreloadResource(action) => unimplemented_action! { action },
        Action::Preload(action) => unimplemented_action! { action },
        Action::EnableAI(action) => unimplemented_action! { action },
        Action::SwitchDataLayers(action) => unimplemented_action! { action },
        Action::DestroyQuest(action) => unimplemented_action! { action },
        Action::DestroyQuestItem(action) => unimplemented_action! { action },
        Action::PromptQuestChapterUI(action) => unimplemented_action! { action },
        Action::TakePlotPhoto(action) => unimplemented_action! { action },
        Action::SetWuYinQuState(action) => unimplemented_action! { action },
        Action::RunActions(action) => unimplemented_action! { action },
        Action::ManualOccupations(action) => unimplemented_action! { action },
        Action::SetWeather(action) => unimplemented_action! { action },
        Action::SendNpcMail(action) => unimplemented_action! { action },
        Action::EnableFunction(action) => unimplemented_action! { action },
        Action::FocusOnMapMark(action) => unimplemented_action! { action },
        Action::CharacterLookAt(action) => unimplemented_action! { action },
        Action::AddGuestCharacter(action) => unimplemented_action! { action },
        Action::RemoveGuestCharacter(action) => unimplemented_action! { action },
        Action::TeleportToAndEnterVehicle(action) => unimplemented_action! { action },
        Action::SetAreaTimeState(action) => unimplemented_action! { action },
        Action::ResetPlayerCameraFocus(action) => unimplemented_action! { action },
        Action::ResetLevelPlay(action) => unimplemented_action! { action },
        Action::VehicleSprint(action) => unimplemented_action! { action },
        Action::VehicleMoveWithPathLine(action) => unimplemented_action! { action },
        Action::ClientPreEnableSubLevels(action) => unimplemented_action! { action },
        Action::GuestOperateUiAnimation(action) => unimplemented_action! { action },
        Action::ChangeEntityCamp(action) => unimplemented_action! { action },
        Action::NewMoveWithSpline(action) => unimplemented_action! { action },
        Action::DangoAbyssActivatePortal(action) => unimplemented_action! { action },
        Action::DangoAbyssCreateRewardTreasureBox(action) => unimplemented_action! { action },
        Action::DangoAbyssGotoNextFloor(action) => unimplemented_action! { action },
        Action::DangoAbyssReceiveReward(action) => unimplemented_action! { action },
        Action::SummonEntity(action) => unimplemented_action! { action },
        Action::GetRewardByInteract(action) => unimplemented_action! { action },
        Action::OpenQte(action) => unimplemented_action! { action },
        Action::ActiveAntiGravitySafePoint(action) => unimplemented_action! { action },
        Action::BvbPlayDialog(action) => unimplemented_action! { action },
        Action::BvbSendSystemEvent(action) => unimplemented_action! { action },
        Action::BvbSendAiEvent(action) => unimplemented_action! { action },
        Action::BvbPlayerOperationConstraint(action) => unimplemented_action! { action },
        Action::ExecClientBattleAction(action) => unimplemented_action! { action },
        Action::TriggerSpecificScanEffect(action) => unimplemented_action! { action },
        Action::SetActorVar(action) => unimplemented_action! { action },
        Action::RunActorCustomEvent(action) => unimplemented_action! { action },
        Action::StopUiScreenEffect(action) => unimplemented_action! { action },
        Action::StopNewMoveWithSpline(action) => unimplemented_action! { action },
        Action::RequestSystemFunction(action) => unimplemented_action! { action },
    }
}

fn collect_action(player: &mut Player,
                  level_entity_data: &wicked_waifus_data::LevelEntityConfigData,
                  template_config: &wicked_waifus_data::TemplateConfigData) {
    if let Some(reward_component) = level_entity_data.components_data.reward_component
        .as_ref()
        .or(template_config.components_data.reward_component.as_ref()) {
        if reward_component.disabled.unwrap_or(false) {
            return;
        }
        // TODO: check the use of reward_type and drop_on_event
        //      Seems type 0 is reward from preview, while 1 and 2 is unknown
        if let Some(reward_id) = reward_component.reward_id {
            let drop = wicked_waifus_data::drop_package_data::get(&reward_id).unwrap();
            let usages = drop.drop_preview.iter()
                .map(|(&id, &quantity)| ItemUsage { id, quantity })
                .collect::<Vec<_>>();
            let updated_items = player.inventory.add_items(&usages);
            let normal_item_list = player.inventory.to_normal_item_list_filtered(
                &updated_items.keys().cloned().collect::<Vec<i32>>()
            );
            player.notify(NormalItemUpdateNotify { normal_item_list, no_tips: false });
            // UpdateHandBookActiveStateMapNotify
            let mut rewards: HashMap<i32, WR> = HashMap::new();
            rewards.insert(0, WR {
                item_list: drop.drop_preview.iter()
                    .map(|(&id, &quantity)| RewardItemInfo {
                        show_plan_id: 0, // TODO: Check how to get this
                        item_id: id,
                        count: quantity,
                        incr_id: 0,
                    })
                    .collect::<Vec<_>>(),
            });
            player.notify(ItemRewardNotify {
                drop_id: reward_id,
                reason: 15000,
                magnification: 1,
                reward_items: rewards,
            });
        }
        // TODO: Should we remove entity?? get pcap
    }
}

#[inline(always)]
fn unlock_teleport_trigger(player: &mut Player, action: UnlockTeleportTrigger) {
    player.unlock_teleport(action.teleport_id)
}

fn change_self_entity_state(player: &mut Player,
                            entity_id: i64,
                            level_entity_data: &wicked_waifus_data::LevelEntityConfigData,
                            template_config: &wicked_waifus_data::TemplateConfigData,
                            action: ChangeSelfEntityState) {
    let state = tag_utils::get_tag_id_by_name(action.entity_state.as_str());

    // TODO: update Tag::CommonEntityTags too??
    let old_state = {
        let world_ref = player.world.borrow();
        let world = world_ref.get_world_entity();
        let mut state_tag = query_components!(world, entity_id, StateTag).0.unwrap();
        let old_state = state_tag.state_tag_id;
        tracing::debug!("ChangeSelfEntityState: old state {old_state} -> new state: {state}");
        state_tag.state_tag_id = state;
        old_state
    };

    if let Some(entity_state_component) = level_entity_data.components_data.entity_state_component.as_ref()
        .or(template_config.components_data.entity_state_component.as_ref()).cloned() {
        let entity_state_component: EntityStateComponent = entity_state_component;  // TODO: Remove this line, used for casting only

        // TODO: implement rest of cases
        if let Some(state_change_behaviors) = entity_state_component.state_change_behaviors {
            for state_change_behavior in state_change_behaviors {
                // TODO: implement rest of cases
                let expected = tag_utils::get_tag_id_by_name(state_change_behavior.state.as_str());

                if expected == state {
                    if let Some(actions) = state_change_behavior.action {
                        for sub in actions {
                            perform_action(player, entity_id, level_entity_data, template_config, sub);
                        }
                    }
                }
            }
        }
    }

    player.notify(EntityCommonTagNotify {
        id: entity_id,
        tags: vec![
            CommonTagData { tag_id: old_state, remove_tag_ids: false }, // Remove
            CommonTagData { tag_id: state, remove_tag_ids: true }, // Add
        ],
    });

    player.notify(EntityStateReadyNotify {
        entity_id,
        tag_id: state,
        ready: true, // TODO: Always true? or shall we compare it to something??
    });
}