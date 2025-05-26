use crate::logic::player::Player;
use wicked_waifus_protocol::{
    ErrorCode, FriendAllRequest, FriendAllResponse, FriendInfo, PlayerBasicInfoGetRequest,
    PlayerBasicInfoGetResponse, PlayerDetails,
};

pub fn on_friend_all_request(
    _player: &Player,
    _: FriendAllRequest,
    response: &mut FriendAllResponse,
) {
    // TODO: Fill from player
    response.friend_apply_list = vec![];
    // TODO: Fill from player
    let mut friends = vec![];
    friends.push(FriendInfo {
        info: Some(get_bot_details()),
        remark: "".to_string(),
    });

    response.friend_info_list = friends;
    response.error_code = ErrorCode::Success.into();
}

// pub fn on_friend_apply_send_request(
//     _player: &Player,
//     _request: FriendApplySendRequest,
//     _response: &mut FriendApplySendResponse,
// ) {
//
// }
//
// pub fn on_friend_recently_team_request(
//     _player: &Player,
//     _request: FriendRecentlyTeamRequest,
//     _response: &mut FriendRecentlyTeamResponse,
// ) {
//
// }

pub fn on_player_basic_info_get_request(
    _player: &Player,
    request: PlayerBasicInfoGetRequest,
    response: &mut PlayerBasicInfoGetResponse,
) {
    if request.id == 1337 {
        response.info = Some(get_bot_details());
        response.error_code = ErrorCode::Success.into();
    } else {
        // TODO: Search in database
        response.error_code = ErrorCode::GmErrPlayerNotFound.into();
    }
}

fn get_bot_details() -> PlayerDetails {
    PlayerDetails {
        player_id: 1337,
        name: "BestWaifu".to_string(),
        level: 90,
        origin_world_level: 8,
        cur_world_level: 8,
        head_id: 82001203,
        head_frame_id: 0,
        signature: "Ara Ara ~".to_string(),
        is_online: true,
        is_can_lobby_online: false,
        last_offline_time: wicked_waifus_commons::time_util::unix_timestamp() as i64,
        team_member_count: 0,
        level_gap: 0,
        birthday: 0,
        role_show_list: vec![],
        card_show_list: vec![],
        cur_card: 0,
        display_birthday: false,
        y0a: 0,
        sdk_user_id: "Encore_PS5".to_string(),
        sdk_online_id: "Encore_PS5".to_string(),
        sdk_account_id: "Encore_PS5".to_string(),
        cross_play_enabled: true,
        limit_state: 0,
        player_title_id: 0,
        cur_player_title_id: 0,
        sex: 0,
    }
}
