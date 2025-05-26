use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::UNIX_EPOCH;

use wicked_waifus_data::{gacha_pool_data, gacha_view_info_data, GachaPoolData, text_map_data};
use wicked_waifus_data::GachaViewTypeInfoId::{BeginnersChoiceConvene, NoviceConvene};
use wicked_waifus_protocol::{ErrorCode, GachaConsume, GachaInfo, GachaInfoRequest, GachaInfoResponse,
                           GachaPoolInfo, GachaRequest, GachaResponse, GachaResult, GachaReward,
                           GachaUsePoolRequest, GachaUsePoolResponse, WeaponItem};

use crate::logic::gacha::gacha_pool::GachaPool;
use crate::logic::gacha::pool_info::PoolInfo;
use crate::logic::gacha::service::GachaService;
use crate::logic::player::Player;

static GACHA_SERVICE: OnceLock<Mutex<GachaService>> = OnceLock::new();

pub fn on_gacha_request(
    player: &mut Player,
    request: GachaRequest,
    response: &mut GachaResponse,
) {
    let mut gacha_service = GACHA_SERVICE.get_or_init(|| Mutex::new(GachaService::new())).lock().unwrap();

    // TODO: ensure we have enough elements before pulling

    match gacha_service.pull(player, request.gacha_id, request.gacha_times) {
        Ok(results) => {
            match consume_tides(player, request.gacha_id, request.gacha_times) {
                Ok(_) => {
                    let _summary = process_gacha_results(&results);
                    //update_player_inventory(player, summary);

                    response.error_code = ErrorCode::Success as i32;
                    response.gacha_results = results;
                }
                Err(error_code) => {
                    response.error_code = error_code as i32;
                    response.gacha_results = Vec::new();
                }
            }
        }
        Err(error_code) => {
            response.error_code = error_code as i32;
            response.gacha_results = Vec::new();
        }
    }
}

pub fn on_gacha_info_request(
    _player: &Player,
    request: GachaInfoRequest,
    response: &mut GachaInfoResponse,
) {
    tracing::debug!("received gacha request for language: {}", request.language);
    let text_map = text_map_data::get_textmap(request.language);

    let gacha_service = GACHA_SERVICE.get_or_init(|| Mutex::new(GachaService::new())).lock().unwrap();
    let active_pools = gacha_service.get_active_pools();

    response.gacha_infos = active_pools.into_iter()
        .filter_map(|(pool_id, pool)| {
            create_gacha_info(pool_id, pool, text_map)
        })
        .collect();

    response.error_code = ErrorCode::Success.into();
    response.daily_total_left_times = -1;
    response.record_id = "ad2aa5a111e34a47cae88bd3ded73f1a".to_string(); // TODO: ??

    // let (name, value) = ("GachaInfoResponse", serde_json::to_string_pretty(response).unwrap());
    // tracing::debug!("trying to log unhandled data for message {name} with:\n{value}")
}

pub fn on_gacha_use_pool_request(
    _player: &Player,
    _request: GachaUsePoolRequest,
    response: &mut GachaUsePoolResponse,
) {
    // TODO: verify that gacha pool exists?
    response.error_code = ErrorCode::Success.into();
}

fn create_gacha_info(pool_id: i32,
                     pool: &GachaPool,
                     textmap: &HashMap<String, String>) -> Option<GachaInfo> {
    let pools: Vec<GachaPoolInfo> = gacha_pool_data::iter()
        .filter(|p| p.gacha_id == pool_id)
        .filter_map(|p| create_pool_info(p, &pool.info, textmap))
        .collect();

    if pools.is_empty() {
        return None;
    }

    let gacha_consumes = handle_gacha_consumes(&pool.info);

    let gacha_info = GachaInfo {
        id: pool_id,
        today_times: pool.daily_pulls,
        total_times: pool.total_pulls,
        item_id: pool.info.item_id,
        gacha_consumes,
        use_pool_id: pools[0].id,
        pools,
        begin_time: pool.info.start_time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
        end_time: pool.info.end_time.map_or(0, |end_time| end_time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64),
        daily_limit_times: pool.info.daily_limit,
        total_limit_times: pool.info.total_limit,
        resources_id: pool.info.resources_id().to_string(),
    };

    Some(gacha_info)
}

fn create_pool_info(
    pool: &GachaPoolData,
    pool_info: &PoolInfo,
    textmap: &HashMap<String, String>,
) -> Option<GachaPoolInfo> {
    // TODO: debug textmap logic
    gacha_view_info_data::iter()
        .find(|view| view.id == pool.id)
        .map(|view| {
            GachaPoolInfo {
                id: pool.id,
                begin_time: pool_info.start_time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
                end_time: pool_info.end_time.map_or(0, |end_time| end_time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64),
                title: textmap.get(&view.summary_title).unwrap_or(&view.summary_title).to_string(),
                description: textmap.get(&view.summary_describe).unwrap_or(&view.summary_describe).to_string(),
                ui_type: view.r#type as i32,
                theme_color: view.theme_color.clone(),
                show_id_list: view.show_id_list.clone(),
                up_list: view.up_list.clone(),
                preview_id_list: view.preview_id_list.clone(),
            }
        })
}

fn handle_gacha_consumes(pool_info: &PoolInfo) -> Vec<GachaConsume> {
    match (pool_info.pool_type, pool_info.pool_id) {
        (NoviceConvene, _) => vec![GachaConsume { times: 10, consume: 0 }], // 8
        (BeginnersChoiceConvene, 51..56) => vec![GachaConsume { times: 1, consume: 0 }], // 1,
        (_, _) => vec![
            GachaConsume { times: 1, consume: 0 }, // 1
            GachaConsume { times: 10, consume: 0 }, // 10
        ],
    }
}

#[derive(Default)]
struct RewardSummary {
    normal_items: HashMap<i32, i32>,
    weapons: Vec<WeaponItem>,
}

fn process_gacha_results(results: &[GachaResult]) -> RewardSummary {
    let mut summary = RewardSummary::default();

    for result in results {
        if let Some(reward) = &result.gacha_reward {
            process_reward(reward, &mut summary);
        }
        for extra_reward in &result.extra_rewards {
            process_reward(extra_reward, &mut summary);
        }
    }

    summary
}

fn process_reward(reward: &GachaReward, summary: &mut RewardSummary) {
    if (20000000..30000000).contains(&reward.item_id) {
        // weapon
        summary.weapons.push(WeaponItem {
            id: reward.item_id,
            incr_id: summary.weapons.len() as i32 + 1,
            func_value: 0,
            weapon_level: 1,
            weapon_exp: 0,
            weapon_breach: 0,
            weapon_reson_level: 1,
            ..Default::default()
        });
    } else {
        // normal item
        *summary.normal_items.entry(reward.item_id).or_insert(0) += reward.item_count;
    }
}

// fn update_player_inventory(player: &mut Player, summary: RewardSummary) {
//     if !summary.normal_items.is_empty() {
//         let items: Vec<(i32, i32)> = summary.normal_items.into_iter().collect();
//         player.add_items(&items);
//     }
//
//     if !summary.weapons.is_empty() {
//         player.add_weapons(summary.weapons);
//     }
// }

/*

50001 - Lustrous tide (standard)
50002 - Radiant tide (character)
50005 - Forging tide (weapon)
50006 - Voucher of Reciprocal Tides (special any default char)
 */
fn consume_tides(_player: &mut Player, pool_id: i32, pull_count: i32) -> Result<(), ErrorCode> {
    let (_currency_id, _actual_cost) = match pool_id {
        1 => {
            // novice with 20% discount
            let discounted_cost = (pull_count as f32 * 0.8).ceil() as i32;
            (50001, discounted_cost)
        }
        2 | 31..=35 | 41..=45 => (50001, pull_count), // Standard, permanent weapon, and beginner character -> Lustrous Tide
        100001..=100100 => (50002, pull_count), // Character -> Radiant Tide
        200001..=200100 => (50005, pull_count), // Weapon -> Forging Tide
        51..56 => (50006, pull_count), // Special -> Voucher of Reciprocal Tides
        _ => return Err(ErrorCode::ErrGachaPoolConfigNotFound),
    };

    // let current_tides = player.item_storage.resources.get(&currency_id).cloned().unwrap_or(0);
    // if current_tides < actual_cost {
    //     return Err(ErrorCode::ErrItemNotEnough);
    // }

    //player.remove_item(currency_id, actual_cost);

    Ok(())
}