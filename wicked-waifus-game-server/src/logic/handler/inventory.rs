use wicked_waifus_protocol::{ItemExchangeInfo, ItemExchangeInfoRequest, ItemExchangeInfoResponse, NormalItemRequest, NormalItemResponse, PhantomItemRequest, PhantomItemResponse, WeaponItemRequest, WeaponItemResponse};

use crate::logic::player::Player;

pub fn on_normal_item_request(
    player: &mut Player,
    _: NormalItemRequest,
    response: &mut NormalItemResponse,
) {
    tracing::debug!("Received NormalItemRequest, returning player inventory");
    response.normal_item_list = player.inventory.to_normal_item_list();
}

pub fn on_weapon_item_request(
    player: &mut Player,
    _: WeaponItemRequest,
    response: &mut WeaponItemResponse,
) {
    response.weapon_item_list = player.inventory.to_weapon_item_list();
}

pub fn on_phantom_item_request(
    _player: &mut Player,
    _: PhantomItemRequest,
    _response: &mut PhantomItemResponse,
) {
    // TODO: Implement this
    tracing::warn!("Unhandled PhantomItemRequest");
}

pub fn on_item_exchange_info_request(
    _player: &mut Player,
    _: ItemExchangeInfoRequest,
    response: &mut ItemExchangeInfoResponse,
) {
    response.item_exchange_infos = wicked_waifus_data::item_exchange_content_data::iter()
        .map(|item_exchange_content_data| ItemExchangeInfo {
            item_id: item_exchange_content_data.item_id,
            today_times: 0, // TODO: For stats only, not used for PS so far
            total_times: 0, // TODO: For stats only, not used for PS so far
            daily_limit: 0, // At the time of writing there is no limits
            total_limit: 0, // At the time of writing there is no limits
        })
        .collect();
}
