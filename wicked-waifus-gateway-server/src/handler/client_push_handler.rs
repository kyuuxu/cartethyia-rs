use wicked_waifus_network::ServiceMessage;
use wicked_waifus_protocol::{message::Message, AceAntiDataPush, ExitGamePush, Protobuf, };
use wicked_waifus_protocol_internal::StopPlayerSessionPush;

use crate::session::Session;

use super::game_server_connection;

pub async fn handle_push(
    session: &mut Session,
    msg: Message,
) -> Result<(), crate::session::SessionError> {
    use wicked_waifus_protocol::MessageID;
    match msg.get_message_id() {
        AceAntiDataPush::MESSAGE_ID => on_ace_anti_data_push(session, msg),
        ExitGamePush::MESSAGE_ID => on_exit_game_push(session, msg).await,
        unhandled => tracing::warn!("can't find handler for push with message_id={unhandled}"),
    }

    Ok(())
}

async fn on_exit_game_push(session: &Session, _: Message) {
    use wicked_waifus_protocol_internal::MessageID;
    game_server_connection::push_message(ServiceMessage {
        src_service_id: 0,
        rpc_id: 0,
        message_id: StopPlayerSessionPush::MESSAGE_ID,
        data: StopPlayerSessionPush {
            gateway_session_id: session.get_conv_id(),
        }
        .encode_to_vec()
        .into_boxed_slice(),
    })
    .await;
}

fn on_ace_anti_data_push(session: &Session, mut msg: Message) {
    let Ok(push) = AceAntiDataPush::decode(msg.remove_payload().as_ref()) else {
        tracing::warn!("failed to decode AceAntiDataPush");
        return;
    };

    tracing::debug!(
        "received AceAntiDataPush from session_id={}, data={}",
        session.get_conv_id(),
        hex::encode(&*push.anti_data)
    );
}
