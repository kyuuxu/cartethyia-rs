use std::collections::HashMap;
use wicked_waifus_protocol::{
    ErrorCode, TutorialInfo, TutorialInfoRequest, TutorialInfoResponse, TutorialReceiveRequest,
    TutorialReceiveResponse, TutorialUnlockRequest, TutorialUnlockResponse,
};

use crate::logic::player::Player;

pub fn on_tutorial_info_request(
    player: &mut Player,
    _: TutorialInfoRequest,
    response: &mut TutorialInfoResponse,
) {
    response.unlock_list = player
        .tutorials
        .tutorials
        .iter()
        .map(|tutorial| TutorialInfo {
            id: tutorial.id,
            create_time: tutorial.create_time,
            get_award: tutorial.get_award,
        })
        .collect();
}

pub fn on_tutorial_receive_request(
    player: &mut Player,
    request: TutorialReceiveRequest,
    response: &mut TutorialReceiveResponse,
) {
    let Some(tutorial_data) =
        wicked_waifus_data::guide_tutorial_data::iter().find(|tutorial| tutorial.id == request.id)
    else {
        response.error_code = ErrorCode::GuideTutorialConfigNotFind.into();
        return;
    };

    let Some(tutorial) = player
        .tutorials
        .tutorials
        .iter()
        .find(|tutorial| tutorial.id == request.id) else {

        response.error_code = ErrorCode::GuideTutorialNotUnlock.into();
        return;
    };

    if tutorial.get_award {
        response.error_code = ErrorCode::GuideTutorialIsReceive.into();
        return;
    }

    // TODO: Search the rewards in drop_package
    tracing::debug!("Tutorial receive request with drop: {}", tutorial_data.drop_id);

    // TODO: Fill in the item map
    response.error_code = ErrorCode::Success.into();
    response.item_map = HashMap::new();
}

pub fn on_tutorial_unlock_request(
    player: &mut Player,
    request: TutorialUnlockRequest,
    response: &mut TutorialUnlockResponse,
) {
    let Some(_) =
        wicked_waifus_data::guide_tutorial_data::iter().find(|tutorial| tutorial.id == request.id)
    else {
        response.error_code = ErrorCode::GuideTutorialConfigNotFind.into();
        return;
    };

    if let Some(tutorial) = player
        .tutorials
        .tutorials
        .iter()
        .find(|tutorial| tutorial.id == request.id)
    {
        if tutorial.get_award {
            response.error_code = ErrorCode::GuideTutorialIsReceive.into();
        } else {
            response.error_code = ErrorCode::GuideTutorialIsUnlock.into();
        }
        return;
    }

    let tutorial = player.unlock_tutorial(request.id);
    response.un_lock_info = Some(TutorialInfo {
        id: tutorial.id,
        create_time: tutorial.create_time,
        get_award: tutorial.get_award,
    });
    response.error_code = ErrorCode::Success.into();
}
