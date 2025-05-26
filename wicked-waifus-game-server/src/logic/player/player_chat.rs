use std::collections::hash_map::Entry;
use std::collections::HashMap;

use wicked_waifus_commons::time_util::unix_timestamp_ms;
use wicked_waifus_protocol::{ChatContentProto, ErrorCode, PrivateChatHistoryContentProto};
use wicked_waifus_protocol_internal::{PlayerChatData, PlayerChatHistoryData, PlayerChatRoomData};

pub struct PlayerChat {
    rooms: HashMap<i32, PlayerChatRoomData>,
}

impl PlayerChat {
    const HISTORY_MAX_SIZE: i32 = 10;

    pub fn load_from_save(data: PlayerChatData) -> Self {
        PlayerChat {
            rooms: data.rooms,
        }
    }

    pub fn build_save_data(&self) -> PlayerChatData {
        PlayerChatData {
            rooms: self.rooms.clone(),
        }
    }

    pub fn validate_message(&self,
                            sender: i32,
                            receiver: i32,
                            content_type: i32,
                            original_message: String) -> Result<PlayerChatHistoryData, i32> {
        // TODO: Pass message filters, censored words shall be replaced in message
        let msg_id = uuid::Uuid::new_v4().to_string();
        let filtered_message = original_message.clone();
        let message = PlayerChatHistoryData {
            sender,
            receiver,
            content_type,
            msg_id,
            original_message,
            filtered_message,
            offline_message: false,
            utc_time: unix_timestamp_ms() as i64, // TODO review unit
            psn_account_id: "".to_string(), // TODO: Support PSN niggers, search something??
        };
        Ok(message)
    }

    pub fn add_message(&mut self, uid: i32, data: PlayerChatHistoryData) {
        // TODO: shall we add room here or shall it be already existent?
        match self.rooms.entry(uid) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().history.push(data)
            }
            Entry::Vacant(entry) => {
                entry.insert(PlayerChatRoomData { history: vec![data] });
            }
        };
    }

    pub fn build_chat_content_proto(&self, data: &PlayerChatHistoryData) -> ChatContentProto {
        ChatContentProto {
            sender_uid: data.sender,
            chat_content_type: data.content_type,
            content: data.filtered_message.clone(),
            offline_msg: data.offline_message,
            utc_time: data.utc_time,
            msg_id: data.msg_id.clone(),
            ps_account_id: data.psn_account_id.clone(),
        }
    }

    pub fn build_private_chat_history_content_proto(&self,
                                                    uid: i32,
                                                    start_index: i32) -> Result<PrivateChatHistoryContentProto, i32> {
        let room = self.rooms.get(&uid);
        if room.is_none() {
            return Err(ErrorCode::ErrNoLoadPrivateChatData.into());
        }
        let history = &room.unwrap().history;
        let history_length = history.len() as i32;
        let remaining_elements = history_length - start_index;
        let mut result = if remaining_elements <= 0 {
            PrivateChatHistoryContentProto {
                target_uid: uid,
                chats: vec![],
                history_is_end: true,
                total_nums: 0,
            }
        } else if remaining_elements < Self::HISTORY_MAX_SIZE {
            PrivateChatHistoryContentProto {
                target_uid: uid,
                chats: vec![],
                history_is_end: true,
                total_nums: history_length - start_index,
            }
        } else {
            PrivateChatHistoryContentProto {
                target_uid: uid,
                chats: vec![],
                history_is_end: false,
                total_nums: Self::HISTORY_MAX_SIZE,
            }
        };
        let start_offset = start_index as usize;
        let end_offset = start_offset + result.total_nums as usize;
        for i in start_offset..end_offset {
            result.chats.push(self.build_chat_content_proto(&history[i]));
        }
        Ok(result)
    }
}

impl Default for PlayerChat {
    fn default() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }
}