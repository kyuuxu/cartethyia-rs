use wicked_waifus_protocol::{LordGymInfoRequest, LordGymInfoResponse};

use crate::logic::player::Player;

pub fn on_lord_gym_info_request(
    _player: &Player,
    request: LordGymInfoRequest,
    _response: &mut LordGymInfoResponse,
) {
    tracing::warn!("LordGymInfoRequest unhandled: for {:?}", request);
    // TODO: Implement relational DB(SeaORM?) and fetch the data from the player
}
