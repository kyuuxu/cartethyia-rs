use wicked_waifus_protocol::{AdviceSetRequest, AdviceSetResponse, ErrorCode};
use crate::logic::player::Player;

pub fn on_advice_set_request(
    player: &mut Player,
    request: AdviceSetRequest,
    response: &mut AdviceSetResponse,
) {
    player.advise.is_show = request.is_show;

    response.is_show = request.is_show;
    response.error_code = ErrorCode::Success.into();
}