use wicked_waifus_protocol::{
    AccessPathTimeServerConfigRequest, AccessPathTimeServerConfigResponse, ErrorCode,
    JsPatchNotify, PlayerHeadDataRequest, PlayerHeadDataResponse, SceneLoadingFinishRequest,
    SceneLoadingFinishResponse, SceneTraceRequest, SceneTraceResponse, UpdateSceneDateRequest,
    UpdateSceneDateResponse,
};

const WATER_MASK: &str = include_str!("../../../scripts/watermask-disable.js");
const UID_FIX: &str = include_str!("../../../scripts/uidfix.js");
const CENSORSHIP_FIX: &str = include_str!("../../../scripts/censorshipfix.js");
const DEBUG_DISABLE: &str = include_str!("../../../scripts/debug_disable.js");

use crate::logic::player::Player;

pub fn on_scene_trace_request(
    _player: &Player,
    request: SceneTraceRequest,
    _: &mut SceneTraceResponse,
) {
    tracing::debug!("SceneTraceRequest: trace id {}", request.scene_trace_id);
}

pub fn on_scene_loading_finish_request(
    player: &Player,
    _request: SceneLoadingFinishRequest,
    response: &mut SceneLoadingFinishResponse,
) {
    player.notify(JsPatchNotify {
        content: WATER_MASK.to_string(),
    });
    player.notify(JsPatchNotify {
        content: UID_FIX
            .replace("{PLAYER_USERNAME}", &player.basic_info.name)
            .replace("{SELECTED_COLOR}", "50FC71"),
    });
    player.notify(JsPatchNotify {
        content: CENSORSHIP_FIX.to_string(),
    });
    player.notify(JsPatchNotify {
        content: DEBUG_DISABLE.to_string(),
    });

    // TODO: Implement this if needed
    response.error_code = ErrorCode::Success.into();
}

pub fn on_update_scene_date_request(
    _player: &Player,
    _request: UpdateSceneDateRequest,
    response: &mut UpdateSceneDateResponse,
) {
    // TODO: port this from golang
    response.error_code = ErrorCode::Success.into();
}

pub fn on_access_path_time_server_config_request(
    _player: &Player,
    _request: AccessPathTimeServerConfigRequest,
    response: &mut AccessPathTimeServerConfigResponse,
) {
    // TODO: port this from golang
    response.pb = vec![];
}

pub fn on_player_head_data_request(
    _player: &Player,
    _request: PlayerHeadDataRequest,
    response: &mut PlayerHeadDataResponse,
) {
    // TODO: port this from golang
    response.pi = vec![];
}
