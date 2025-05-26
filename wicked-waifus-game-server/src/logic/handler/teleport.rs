use wicked_waifus_protocol::{ErrorCode, JoinSceneNotify, LeaveSceneNotify, TeleportDataRequest, TeleportDataResponse, TeleportFinishRequest, TeleportFinishResponse, TeleportNotify, TeleportReason, TeleportTransferRequest, TeleportTransferResponse, TransitionOptionPb};

use wicked_waifus_data::{level_entity_config_data, RawVectorData};
use wicked_waifus_data::pb_components::teleport::TeleportComponent;

use crate::logic::math::Vector3f;
use crate::logic::player::Player;
use crate::logic::utils::world_util;

pub fn on_teleport_data_request(
    player: &mut Player,
    _: TeleportDataRequest,
    response: &mut TeleportDataResponse,
) {
    response.error_code = ErrorCode::Success.into();
    response.ids = player.teleports.teleports_data.iter()
        .map(|teleport| teleport.id)
        .collect::<Vec<_>>();
}

pub fn on_teleport_transfer_request(
    player: &mut Player,
    request: TeleportTransferRequest,
    response: &mut TeleportTransferResponse,
) {
    tracing::debug!("received transfer request for teleport id: {}", request.id);
    let Some(teleport) = wicked_waifus_data::teleporter_data::iter()
        .find(|teleporter| request.id == teleporter.id) else {
        response.error_code = ErrorCode::ErrTeleportIdNotExist.into();
        return;
    };

    println!("received transfer request for teleport entity id: {}", &teleport.teleport_entity_config_id);
    let Some(tp) = level_entity_config_data::get(teleport.map_id, teleport.teleport_entity_config_id) else {
        response.error_code = ErrorCode::ErrTeleportEntityNotExist.into();
        return;
    };

    let Some(teleport_component) = &tp.components_data.teleport_component else {
        response.error_code = ErrorCode::ErrTeleportComponentNotExist.into();
        return;
    };

    if teleport_component.disabled.unwrap_or(false) ||
        teleport_component.teleporter_id.is_none() {
        response.error_code = ErrorCode::ErrTeleportGmGetCreatureGenCfgFailed.into();
    }
    if teleport_component.teleporter_id.unwrap() != request.id {
        response.error_code = ErrorCode::ErrTeleportComponentNotMatch.into();
    } else {
        response.error_code = ErrorCode::Success.into();
        response.map_id = teleport.map_id;
        let teleport_position = get_teleport_position(&tp.transform, teleport_component);
        response.pos_x = teleport_position.x;
        response.pos_y = teleport_position.y;
        response.pos_z = teleport_position.z;
        response.pitch = 0f32;
        response.yaw = 0f32;
        response.roll = 0f32;

        if player.basic_info.cur_map_id == teleport.map_id {
            player.notify(TeleportNotify {
                map_id: teleport.map_id,
                pos: Some(teleport_position.to_protobuf()),
                rot: None,
                gravity: None,
                reason: TeleportReason::Gm.into(),
                game_ctx: None,
                transition_option: Some(TransitionOptionPb::default()),
                disable_auto_fade: false,
            });
        } else {
            // remove entity
            player.notify(LeaveSceneNotify {
                player_id: player.basic_info.id,
                scene_id: "".to_string(),
                transition_option: Some(TransitionOptionPb::default()),
            });
            let scene_info = world_util::build_scene_information(&player);
            // TODO: Trigger initial join world flow??
            player.notify(JoinSceneNotify {
                scene_info: Some(scene_info),
                max_entity_id: i64::MAX,
                transition_option: Some(TransitionOptionPb::default()),
            });
        }
    }
}

pub fn on_teleport_finish_request(
    _player: &mut Player,
    _: TeleportFinishRequest,
    response: &mut TeleportFinishResponse,
) {
    // Should we store here if the player is in the correct position?
    //  Use internal OnTeleportProcess??
    response.error_code = ErrorCode::Success.into();
}

fn get_teleport_position(transform: &[RawVectorData], component: &TeleportComponent) -> Vector3f {
    let mut entity_position = Vector3f::from_raw_scaled(&transform[0], &transform[2]);
    if let Some(teleport_position) = &component.teleport_position {
        entity_position.add_teleport_position(teleport_position);
    }
    entity_position
}