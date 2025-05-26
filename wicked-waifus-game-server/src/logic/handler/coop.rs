use wicked_waifus_protocol::{ErrorCode, LobbyListRequest, LobbyListResponse};
use crate::logic::player::Player;

pub fn on_lobby_list_request(
    _player: &mut Player,
    request: LobbyListRequest,
    response: &mut LobbyListResponse,
) {
    match request.is_friend {
        true => {
            tracing::debug!("Requesting list of friends lobbies");
        }
        false => {
            tracing::debug!("Requesting list of open lobbies");
        }
    }
    response.error_code = ErrorCode::Success.into();
}