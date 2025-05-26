use wicked_waifus_protocol::{ErrorCode, GuideFinishRequest, GuideFinishResponse, GuideInfoRequest, GuideInfoResponse, GuideTriggerRequest, GuideTriggerResponse};

use crate::logic::player::Player;

pub fn on_guide_info_request(
    player: &Player,
    _: GuideInfoRequest,
    response: &mut GuideInfoResponse,
) {
    response.guide_group_finish_list = player.guides.finished_guides.iter()
        .cloned()
        .collect();
}

pub fn on_guide_trigger_request(
    player: &mut Player,
    request: GuideTriggerRequest,
    response: &mut GuideTriggerResponse,
) {
    response.error_code = check_if_guide_exists_and_is_repeatable(player, request.group_id);
    if response.error_code == <ErrorCode as Into<i32>>::into(ErrorCode::Success) {
        // TODO: We need to check if guide can be repeated or not
        // if player.guides.started_guides.contains(&request.group_id) {
        //     response.error_code = ErrorCode::GuideGroupDoing.into();
        //     return;
        // }
        player.guides.started_guides.insert(request.group_id);
    }
}

pub fn on_guide_finish_request(
    player: &mut Player,
    request: GuideFinishRequest,
    response: &mut GuideFinishResponse,
) {
    response.error_code = check_if_guide_exists_and_is_repeatable(player, request.group_id);
    if response.error_code == <ErrorCode as Into<i32>>::into(ErrorCode::Success) {
        if !player.guides.started_guides.contains(&request.group_id) {
            response.error_code = ErrorCode::GuideGroupNoClient.into();
            return;
        }
        player.guides.started_guides.remove(&request.group_id);
        player.guides.finished_guides.insert(request.group_id);
    }
}

fn check_if_guide_exists_and_is_repeatable(player: &Player, guide_id: i32) -> i32 {
    let Some(guide) = wicked_waifus_data::guide_group_data::iter().find(|guide| guide.id == guide_id) else {
        return ErrorCode::GuideGroupIdNoMatch.into();
    };
    // TODO: We need to check if guide can be repeated or not
    if player.guides.finished_guides.contains(&guide.id) {
        return ErrorCode::GuideGroupIsNotRepeat.into();
    }
    ErrorCode::Success.into()
}