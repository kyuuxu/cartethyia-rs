use std::time::SystemTime;

use wicked_waifus_data::GachaViewTypeInfoId;
use wicked_waifus_data::GachaViewTypeInfoId::{
    BeginnersChoiceConvene, FeaturedResonatorConvene, FeaturedWeaponConvene,
    MultipleChoiceResonatorConvene, MultipleChoiceWeaponConvene, NoviceConvene,
    StandardResonatorConvene, StandardWeaponConvene,
};

use crate::logic::gacha::category::PoolCategory;

pub(crate) struct PitySystem {
    pub soft_pity_start: i32,
    pub hard_pity_four: i32,
    pub hard_pity_five: i32,
}

impl Default for PitySystem {
    fn default() -> Self {
        Self {
            soft_pity_start: 60,
            hard_pity_four: 10,
            hard_pity_five: 80,
        }
    }
}

impl PitySystem {
    fn novice() -> Self {
        Self {
            soft_pity_start: 40,
            hard_pity_four: 10,
            hard_pity_five: 50,
        }
    }
}

pub struct PoolInfo {
    pub pool_id: i32,
    pub pool_type: GachaViewTypeInfoId,
    pub category: PoolCategory,
    pub start_time: SystemTime,
    pub end_time: Option<SystemTime>,
    pub item_id: i32,
    pub daily_limit: i32,
    pub total_limit: i32,
    pub rate_up_five_star: Vec<i32>, // TODO: Instead of copying the vec, use references
    pub rate_up_four_star: Vec<i32>, // TODO: Instead of copying the vec, use references
    pub guaranteed_character_id: Option<i32>,
    pub pity_system: PitySystem,
}

impl PoolInfo {
    const FEATURED_RESONATOR_GACHA_POOL_RESOURCE: &'static str = "UiItem_RoleUpGachaPool";
    const FEATURED_WEAPON_GACHA_POOL_RESOURCE: &'static str = "UiItem_WeaponGachaPool";
    const NOVICE_GACHA_POOL_RESOURCE: &'static str = "UiItem_NewPlayerGachaPool";
    const BASE_GACHA_POOL_RESOURCE: &'static str = "UiItem_BaseGachaPool";

    pub(crate) fn new(
        pool_id: i32,
        pool_type: GachaViewTypeInfoId,
        category: PoolCategory,
        rate_up_five_star: &[i32],
        rate_up_four_star: &[i32],
        guaranteed_character_id: Option<i32>,
    ) -> Self {
        let start_time = SystemTime::now();
        let end_time = match category {
            PoolCategory::Permanent => None,
            PoolCategory::Event(duration) | PoolCategory::Special(duration) => {
                Some(start_time + duration)
            }
        };

        // TODO: Make objects const 50001, 50002, 50005, 50006 or check if gacha consumes exist
        let (item_id, daily_limit, total_limit, pity_system) = match pool_type {
            NoviceConvene => (50001, 0, 50, PitySystem::novice()),
            StandardResonatorConvene | StandardWeaponConvene => {
                (50001, 0, 80, PitySystem::default())
            }
            // TODO: Review MultipleChoiceConvene
            FeaturedResonatorConvene
            | MultipleChoiceResonatorConvene
            | MultipleChoiceWeaponConvene => (50002, 0, 0, PitySystem::default()),
            FeaturedWeaponConvene => (50005, 0, 0, PitySystem::default()),
            BeginnersChoiceConvene => match pool_id {
                51..56 => (50006, 0, 1, PitySystem::default()),
                _ => (50001, 0, 80, PitySystem::default()),
            },
        };

        Self {
            pool_id,
            pool_type,
            category,
            start_time,
            end_time,
            item_id,
            daily_limit,
            total_limit,
            rate_up_five_star: rate_up_five_star.to_vec(), // TODO: Instead of copying the vec, use references
            rate_up_four_star: rate_up_four_star.to_vec(), // TODO: Instead of copying the vec, use references
            guaranteed_character_id,
            pity_system,
        }
    }

    pub(crate) fn is_active(&self) -> bool {
        let now = SystemTime::now();
        match self.end_time {
            Some(end_time) => now >= self.start_time && now < end_time,
            None => now >= self.start_time,
        }
    }

    pub(crate) fn resources_id(&self) -> &str {
        let result = match self.pool_type {
            FeaturedResonatorConvene => Self::FEATURED_RESONATOR_GACHA_POOL_RESOURCE,
            FeaturedWeaponConvene => Self::FEATURED_WEAPON_GACHA_POOL_RESOURCE,
            NoviceConvene => Self::NOVICE_GACHA_POOL_RESOURCE,
            StandardResonatorConvene | StandardWeaponConvene => Self::BASE_GACHA_POOL_RESOURCE,
            _ => Self::BASE_GACHA_POOL_RESOURCE,
        };
        result
    }
}
