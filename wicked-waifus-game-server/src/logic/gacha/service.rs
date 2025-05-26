use std::collections::HashMap;
use std::time::Duration;

use rand::prelude::StdRng;
use rand::SeedableRng;

use wicked_waifus_data::gacha_view_info_data;
use wicked_waifus_data::GachaViewTypeInfoId::{
    BeginnersChoiceConvene, FeaturedResonatorConvene, FeaturedWeaponConvene,
    MultipleChoiceResonatorConvene, MultipleChoiceWeaponConvene, NoviceConvene,
    StandardResonatorConvene, StandardWeaponConvene,
};
use wicked_waifus_protocol::{ErrorCode, GachaResult};

use crate::logic::gacha::category::PoolCategory;
use crate::logic::gacha::gacha_pool::GachaPool;
use crate::logic::gacha::pool_info::PoolInfo;
use crate::logic::player::Player;

pub struct GachaService {
    pools: HashMap<i32, GachaPool>,
    rng: StdRng,
}

impl GachaService {
    const THREE_WEEKS: Duration = Duration::from_secs(3 * 7 * 24 * 60 * 60);
    const ONE_WEEK: Duration = Duration::from_secs(7 * 24 * 60 * 60);

    pub fn new() -> Self {
        Self {
            pools: Self::initialize_default_pools(),
            rng: StdRng::from_os_rng(),
        }
    }

    fn initialize_default_pools() -> HashMap<i32, GachaPool> {
        let mut pools = HashMap::new();

        for element in gacha_view_info_data::iter() {
            let duration = match element.r#type {
                NoviceConvene | StandardResonatorConvene | StandardWeaponConvene => {
                    PoolCategory::Permanent
                }
                // TODO: Review MultipleChoiceConvene
                FeaturedResonatorConvene
                | FeaturedWeaponConvene
                | MultipleChoiceResonatorConvene
                | MultipleChoiceWeaponConvene => PoolCategory::Event(Self::THREE_WEEKS),
                BeginnersChoiceConvene => match element.id {
                    51..56 => PoolCategory::Special(Self::ONE_WEEK),
                    _ => PoolCategory::Permanent,
                },
            };
            let guaranteed = if (element.show_id_list.len() > 0)
                && (element.r#type == FeaturedResonatorConvene)
            {
                Some(element.show_id_list[0])
            } else {
                None
            };
            let info = PoolInfo::new(
                element.id,
                element.r#type,
                duration,
                &element.up_list[..],
                &element.show_id_list[..],
                guaranteed,
            );
            pools.insert(element.id, GachaPool::new(info));
        }
        pools
    }

    pub fn pull(
        &mut self,
        player: &mut Player,
        pool_id: i32,
        times: i32,
    ) -> Result<Vec<GachaResult>, ErrorCode> {
        let pool = self
            .pools
            .get_mut(&pool_id)
            .ok_or(ErrorCode::ErrGachaPoolConfigNotFound)?;

        if !pool.is_active() {
            return Err(ErrorCode::ErrGachaPoolIsNotOpen);
        }

        // TODO: maybe implement ErrGachaPoolIsNotInOpenTime to check if the pool is within its open time

        let mut results = Vec::new();
        for _ in 0..times {
            match pool.pull(&mut self.rng, player) {
                Ok(result) => results.push(result),
                Err(error_code) => return Err(error_code),
            }
        }

        Ok(results)
    }

    pub fn get_active_pools(&self) -> Vec<(i32, &GachaPool)> {
        self.pools
            .iter()
            .filter(|(_, pool)| pool.is_active())
            .map(|(id, pool)| (*id, pool))
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_all_pools(&self) -> Vec<(i32, &PoolInfo)> {
        self.pools
            .iter()
            .map(|(id, pool)| (*id, &pool.info))
            .collect()
    }
}
