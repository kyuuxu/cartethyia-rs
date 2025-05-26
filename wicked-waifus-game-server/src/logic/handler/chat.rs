use tracing::debug;

use wicked_waifus_protocol::{ErrorCode, PrivateChatDataRequest, PrivateChatDataResponse, PrivateChatHistoryRequest, PrivateChatHistoryResponse, PrivateChatOperateRequest, PrivateChatOperateResponse, PrivateChatOperateType, PrivateChatRequest, PrivateChatResponse};

use crate::logic::player::Player;

pub fn on_private_chat_request(
    player: &mut Player,
    request: PrivateChatRequest,
    response: &mut PrivateChatResponse,
) {
    let own_id = player.basic_info.id;
    // TODO: Implement block and ban checks?? Ignore them for the time being
    let result = player.player_chat.validate_message(
        own_id,
        request.target_uid,
        request.chat_content_type,
        request.content,
    );
    match result {
        Ok(message) => {
            player.player_chat.add_message(own_id, message.clone());
            // TODO: Check how to search a player from a different world(db search or session search)
            // let other_player = ...;
            // let other_player_message = message.clone();
            // TODO: change offline msg depending on the player connection
            // other_player.player_chat.add_message(other_player.basic_info.id, other_player_message);

            response.target_uid = request.target_uid;
            response.error_code = ErrorCode::Success.into();
            response.msg_id = message.msg_id.clone();
            response.filter_msg = message.filtered_message.clone();
            // TODO: If the other player is online, notify message
            // player.notify(PrivateMessageNotify {
            //     chat_content: Some(
            //         player.player_chat.build_chat_content_proto(other_player_message)
            //     )
            // })
        }
        Err(error_code) => response.error_code = error_code
    };
}

pub fn on_private_chat_data_request(
    _: &Player,
    _: PrivateChatDataRequest,
    _: &mut PrivateChatDataResponse,
) {

}

pub fn on_private_chat_history_request(
    player: &Player,
    request: PrivateChatHistoryRequest,
    response: &mut PrivateChatHistoryResponse,
) {
    match player.player_chat.build_private_chat_history_content_proto(
        request.target_uid,
        request.start_index,
    ) {
        Ok(chat_history_content_proto) => {
            response.error_code = ErrorCode::Success.into();
            response.data = Some(chat_history_content_proto)
        }
        Err(error_code) => response.error_code = error_code
    }
}


pub fn on_private_chat_operate_request(
    _player: &Player,
    request: PrivateChatOperateRequest,
    response: &mut PrivateChatOperateResponse,
) {
    let operate_type = PrivateChatOperateType::try_from(request.operate_type).unwrap();
    if operate_type == PrivateChatOperateType::ReadMsg && request.target_player_id == 0 {
        // TODO: Additional actions?
        response.error_code = ErrorCode::Success.into();
    } else {
        // TODO: Additional checks
        debug!("on_private_chat_operate_request called for unimplemented case: {:?}", request);
        response.error_code = ErrorCode::Success.into();
    }
}