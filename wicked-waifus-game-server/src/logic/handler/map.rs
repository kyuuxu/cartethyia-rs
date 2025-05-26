use wicked_waifus_protocol::{
    DarkCoastDeliveryRequest, DarkCoastDeliveryResponse, DragonPoolDropItems, EntityAccessInfo,
    ErrorCode, ItemDict, ItemEntry, MapCancelTraceRequest, MapCancelTraceResponse,
    MapTraceInfoRequest, MapTraceInfoResponse, MapTraceRequest, MapTraceResponse,
    MapUnlockFieldInfoRequest, MapUnlockFieldInfoResponse, PlayerAccessEffectAreaRequest,
    PlayerAccessEffectAreaResponse,
};

use crate::logic::player::Player;

pub fn on_dark_coast_delivery_request(
    _player: &mut Player,
    request: DarkCoastDeliveryRequest,
    response: &mut DarkCoastDeliveryResponse,
) {
    // TODO: [WWPS-1] Real implementation should fetch completed / uncompleted from db, lets return completed
    match wicked_waifus_data::dragon_pool_data::get(&request.dragon_pool_id) {
        None => response.error_code = ErrorCode::ErrDragonPoolConf.into(),
        Some(value) => {
            response.error_code = ErrorCode::Success.into();
            response.level_gain = value.core_id;
            response.defeated_guard = value.goal.clone();
            // response.received_guard_reward =
            response.dragon_pool_drop_items = Some(DragonPoolDropItems {
                dragon_pool_id: request.dragon_pool_id,
                q_ss: value.dark_coast_delivery_list.clone(),
                drop_items: vec![ItemDict {
                    items: value
                        .drop_ids
                        .iter()
                        .map(|id| ItemEntry {
                            item_id: *id,
                            item_count: 1,
                        })
                        .collect::<Vec<_>>(),
                }],
            })
        }
    }
}

pub fn on_map_cancel_trace_request(
    player: &mut Player,
    request: MapCancelTraceRequest,
    response: &mut MapCancelTraceResponse,
) {
    player.map_trace.traces.remove(&request.mark_id);
    response.mark_id = request.mark_id;
    response.error_code = ErrorCode::Success.into();
}

pub fn on_map_trace_request(
    player: &mut Player,
    request: MapTraceRequest,
    response: &mut MapTraceResponse,
) {
    player.map_trace.traces.insert(request.mark_id);
    response.mark_id = request.mark_id;
    response.error_code = ErrorCode::Success.into();
}

pub fn on_map_trace_info_request(
    player: &Player,
    _: MapTraceInfoRequest,
    response: &mut MapTraceInfoResponse,
) {
    response.mark_id_list = player.map_trace.traces.iter().cloned().collect();
    response.error_code = ErrorCode::Success.into();
}

pub fn on_map_unlock_field_info_request(
    _player: &mut Player,
    _: MapUnlockFieldInfoRequest,
    response: &mut MapUnlockFieldInfoResponse,
) {
    // TODO: [WWPS-1] Real implementation should fetch completed / uncompleted from db, lets return completed
    response.error_code = ErrorCode::Success.into();
    response.field_id = wicked_waifus_data::area_data::iter()
        .map(|area| area.area_id)
        .collect::<Vec<_>>();
}

pub fn on_player_access_effect_area_request(
    _player: &Player,
    request: PlayerAccessEffectAreaRequest,
    response: &mut PlayerAccessEffectAreaResponse,
) {
    // TODO: from world fetch entity by request.entity_id
    // TODO: Compute the distance between player and entity.entity_id
    response.error_code = ErrorCode::Success.into();
    response.entity_id = request.entity_id;
    response.info = Some(EntityAccessInfo {
        entity_id: request.entity_id,
        range_type: request.range_type,
        uo_1: Default::default(),
    });
}
