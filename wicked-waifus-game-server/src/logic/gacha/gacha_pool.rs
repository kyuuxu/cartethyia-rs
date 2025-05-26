use rand::prelude::IndexedRandom;
use rand::Rng;
use wicked_waifus_protocol::{ErrorCode, GachaResult, GachaReward};

use wicked_waifus_data::GachaViewTypeInfoId::{
    BeginnersChoiceConvene, FeaturedResonatorConvene, FeaturedWeaponConvene,
    MultipleChoiceResonatorConvene, MultipleChoiceWeaponConvene, NoviceConvene,
    StandardResonatorConvene, StandardWeaponConvene,
};

use crate::logic::gacha::pool_info::PoolInfo;
use crate::logic::player::Player;
use crate::logic::role::Role;

pub struct PoolRates {
    pub three_star: f32,
    pub four_star: f32,
    pub five_star: f32,
}

impl Default for PoolRates {
    fn default() -> Self {
        Self {
            three_star: 93.2,
            four_star: 20.0, // 6.0
            five_star: 30.0, // 0.8
        }
    }
}

pub struct GachaPool {
    pub info: PoolInfo,
    pub rates: PoolRates,
    pub pull_count: i32,
    pub pity_four: i32,
    pub daily_pulls: i32,
    pub total_pulls: i32,
    pub rate_up: bool, // TODO: persistence state within player data
}

impl GachaPool {
    pub(crate) fn new(info: PoolInfo) -> Self {
        Self {
            info,
            rates: PoolRates::default(),
            pull_count: 0,
            pity_four: 0,
            daily_pulls: 0,
            total_pulls: 0,
            rate_up: false,
        }
    }

    pub fn pull<T: Rng>(
        &mut self,
        rng: &mut T,
        player: &mut Player,
    ) -> Result<GachaResult, ErrorCode> {
        self.check_limits()?;

        let result = if (self.info.pool_type == BeginnersChoiceConvene)
            && (self.info.pool_id > 50)
            && (self.info.pool_id < 60)
        {
            let item_id = self.info.guaranteed_character_id.unwrap();
            GachaResult {
                gacha_reward: Some(GachaReward {
                    item_id,
                    item_count: 1,
                }),
                extra_rewards: self.calculate_extra_rewards(2),
                transform_rewards: Self::get_transform_rewards(player, item_id),
                bottom: None,
            }
        } else {
            let rarity = self.determine_rarity(&self.calculate_probabilities(), rng);
            let item_id = match self.info.pool_type {
                FeaturedResonatorConvene => {
                    let item_id = if rarity == 2 {
                        if self.rate_up || rng.random_bool(0.5) {
                            self.rate_up = false;
                            self.info.guaranteed_character_id.unwrap_or(0)
                        } else {
                            self.rate_up = true;
                            *[1203, 1405, 1104, 1503, 1301].choose(rng).unwrap()
                        }
                    } else {
                        self.get_random_item(rarity, rng)
                    };
                    item_id
                }
                _ => self.get_random_item(rarity, rng),
            };
            self.update_pity(rarity);
            GachaResult {
                gacha_reward: Some(GachaReward {
                    item_id,
                    item_count: 1,
                }),
                extra_rewards: self.calculate_extra_rewards(rarity),
                transform_rewards: Self::get_transform_rewards(player, item_id),
                bottom: None,
            }
        };

        self.update_limits();
        Ok(result)
    }

    fn get_transform_rewards(player: &mut Player, item_id: i32) -> Vec<GachaReward> {
        let mut transform_rewards = Vec::new();
        let required_role_ids: Vec<i32> = Role::get_all_roles_except_mc();
        match player.role_list.get(&item_id) {
            None => {
                if required_role_ids.contains(&item_id) {
                    let role = Role::new(item_id);
                    let role_id = role.role_id;
                    let weapon_id = role.equip_weapon;
                    player.role_list.insert(item_id, role);
                    // TODO notifies player update
                    player
                        .inventory
                        .add_weapon(weapon_id, 0, 1, 1, 0, 1, Some(role_id))
                        .unwrap();
                    // TODO notifies weapon update
                }
            }
            Some(role) => {
                // TODO: Even if we have, we can't get more than six wavebands, make a check
                transform_rewards.push(GachaReward {
                    item_id: 10000000 + role.role_id,
                    item_count: 1,
                }) // TODO: get from role data
            }
        }
        transform_rewards
    }

    fn get_random_item(&self, rarity: usize, rng: &mut impl Rng) -> i32 {
        let items: &[i32] = match rarity {
            0 => &[
                21010013, 21020013, 21030013, 21040013, 21050013, 21010023, 21020023, 21030023,
                21040023, 21050023, 21010043, 21020043, 21030043, 21040043, 21050043,
            ],
            1 => match self.info.pool_type {
                StandardWeaponConvene => &[
                    21010024, 21020024, 21030024, 21040024, 21050024, 21010044, 21020044, 21030044,
                    21040044, 21050044, 21010064, 21020064, 21030064, 21040064, 21050064,
                ],
                FeaturedResonatorConvene | FeaturedWeaponConvene => {
                    &self.info.rate_up_four_star[..]
                }
                _ => &[1303, 1602, 1102, 1204, 1403, 1103, 1402, 1202, 1601],
            },
            2 => match self.info.pool_type {
                NoviceConvene | StandardResonatorConvene => &[1405, 1301, 1503, 1104, 1203],
                // TODO: Review MultipleChoiceConvene
                FeaturedResonatorConvene
                | FeaturedWeaponConvene
                | StandardWeaponConvene
                | BeginnersChoiceConvene
                | MultipleChoiceResonatorConvene
                | MultipleChoiceWeaponConvene => &self.info.rate_up_five_star[..],
            },
            _ => unreachable!(),
        };
        *items.choose(rng).unwrap_or(&0)
    }

    fn check_limits(&self) -> Result<(), ErrorCode> {
        if self.daily_pulls >= self.info.daily_limit && self.info.daily_limit != 0 {
            return Err(ErrorCode::ErrGachaDailyTimesLimit);
        }
        if self.total_pulls >= self.info.total_limit && self.info.total_limit != 0 {
            return Err(ErrorCode::ErrGachaTotalTimesLimit);
        }
        Ok(())
    }

    fn update_limits(&mut self) {
        self.daily_pulls = 1;
        self.total_pulls = 1;
    }

    fn calculate_probabilities(&self) -> [f32; 3] {
        let mut prob = [
            self.rates.three_star,
            self.rates.four_star,
            self.rates.five_star,
        ];

        if self.pull_count >= self.info.pity_system.soft_pity_start {
            let extra_prob =
                0.8 + 8.0 * (self.pull_count - self.info.pity_system.soft_pity_start - 1) as f32;
            prob[0] -= extra_prob;
            prob[2] = extra_prob;
        }

        match (
            self.pity_four + 1 >= self.info.pity_system.hard_pity_four,
            self.pull_count + 1 >= self.info.pity_system.hard_pity_five,
        ) {
            (true, _) => [0.0, 100.0, 0.0],
            (_, true) => [0.0, 0.0, 100.0],
            _ => prob,
        }
    }

    fn determine_rarity(&self, prob: &[f32; 3], rng: &mut impl Rng) -> usize {
        let roll: f32 = rng.random_range(0.0..100.0);
        match (roll < prob[2], roll < prob[2] + prob[1]) {
            (true, _) => 2, // 5-star
            (_, true) => 1, // 4-star
            _ => 0,         // 3-star
        }
    }

    fn update_pity(&mut self, rarity: usize) {
        self.pity_four = match rarity {
            1 => 0,
            _ => self.pity_four + 1,
        };

        self.pull_count = match rarity {
            2 => 0,
            _ => self.pull_count + 1,
        };

        // reset rate_up if a 5-star was pulled
        if rarity == 2 {
            self.rate_up = false;
        }
    }

    fn calculate_extra_rewards(&self, rarity: usize) -> Vec<GachaReward> {
        let mut rewards = Vec::new();

        if rarity == 0 {
            rewards.push(GachaReward {
                item_id: 50003, // oscillate corals
                item_count: 15,
            });
        }

        /*
        TODO: update rewards for duplicates

        4-star duplicate:
            1st to 6th duplicate: 3 afterglow corals and 1 waveband of that char
            7th duplicate onwards: 8 afterglow corals
            will not receive any afterglow corals when you pull a 4-star that you do not already own for the first time.

        5-star duplicate:
            1st to 6th duplicate: 15 afterglow corals and 1 waveband of that char
            7th duplicate onwards: 40 afterglow corals
            will not receive any afterglow corals when you pull a 5-star that you do not already own for the first time.
         */
        match rarity {
            2 => rewards.push(GachaReward {
                item_id: 50004, // afterglow corals
                item_count: 15,
            }),
            1 => rewards.push(GachaReward {
                item_id: 50004, // afterglow corals
                item_count: 3,
            }),
            _ => {}
        }

        rewards
    }

    pub fn is_active(&self) -> bool {
        self.info.is_active()
    }
}
