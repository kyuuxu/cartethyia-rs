use wicked_waifus_protocol::{
    player_attr, BasicInfoNotify, PlayerAttr, PlayerAttrKey, PlayerAttrType,
};
use wicked_waifus_protocol_internal::PlayerBasicData;
use crate::logic::player::player_inventory::PlayerInventory;

pub struct PlayerBasicInfo {
    pub id: i32,
    pub name: String,
    pub sex: i32,
    pub level: i32,
    pub exp: i32,
    pub head_photo: i32,
    pub head_frame: i32,
    pub cur_map_id: i32,
    pub role_show_list: Vec<i32>,
}

impl PlayerBasicInfo {
    pub fn build_notify(&self, inventory: &PlayerInventory) -> BasicInfoNotify {
        BasicInfoNotify {
            id: self.id,
            attributes: vec![
                build_int_attr(PlayerAttrKey::Level, self.level),
                build_int_attr(PlayerAttrKey::Exp, self.exp),
                build_int_attr(PlayerAttrKey::Coin, inventory.get_shell_credits()),
                build_int_attr(PlayerAttrKey::RareCoin, inventory.get_astrite()),
                build_int_attr(PlayerAttrKey::HeadPhoto, self.head_photo),
                build_int_attr(PlayerAttrKey::HeadFrame, self.head_frame),
                build_int_attr(PlayerAttrKey::AreaId, 1), // TODO:
                build_str_attr(PlayerAttrKey::Name, self.name.as_str()),
                build_str_attr(PlayerAttrKey::Sign, ""), // TODO:
                build_int_attr(PlayerAttrKey::Sex, self.sex),
                build_int_attr(PlayerAttrKey::OriginWorldLevel, 1), // TODO:
                build_int_attr(PlayerAttrKey::CurWorldLevel, 1), // TODO:
                build_int_attr(PlayerAttrKey::WorldLevelTimeStamp, 0), // TODO:
                build_int_attr(PlayerAttrKey::CashCoin, inventory.get_lunite()),
                build_int_attr(PlayerAttrKey::WorldPermission, 0), // TODO:
            ],
            card_unlock_list: vec![], // TODO: 80060000
            cur_card_id: 0, // TODO: 80060000
            ..Default::default()
        }
    }

    pub fn load_from_save(data: PlayerBasicData) -> Self {
        Self {
            id: data.id,
            name: data.name,
            sex: data.sex,
            level: data.level,
            exp: data.exp,
            head_photo: data.head_photo,
            head_frame: data.head_frame,
            cur_map_id: data.cur_map_id,
            role_show_list: data.role_show_list,
        }
    }

    pub fn build_save_data(&self) -> PlayerBasicData {
        PlayerBasicData {
            id: self.id,
            name: self.name.clone(),
            sex: self.sex,
            level: self.level,
            exp: self.exp,
            head_photo: self.head_photo,
            head_frame: self.head_frame,
            cur_map_id: self.cur_map_id,
            role_show_list: self.role_show_list.clone(),
        }
    }
}

impl Default for PlayerBasicInfo {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            sex: 0,
            level: 0,
            exp: 0,
            head_photo: 0,
            head_frame: 0,
            cur_map_id: 0,
            role_show_list: vec![],
        }
    }
}

#[inline]
fn build_int_attr(key: PlayerAttrKey, value: i32) -> PlayerAttr {
    PlayerAttr {
        key: key.into(),
        value_type: PlayerAttrType::Int32.into(),
        value: Some(player_attr::Value::Int32Value(value)),
    }
}

#[inline]
fn build_str_attr(key: PlayerAttrKey, value: &str) -> PlayerAttr {
    PlayerAttr {
        key: key.into(),
        value_type: PlayerAttrType::String.into(),
        value: Some(player_attr::Value::StringValue(value.to_string())),
    }
}
