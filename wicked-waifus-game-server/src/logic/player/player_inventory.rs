use std::collections::HashMap;
use std::sync::atomic::AtomicI32;
use wicked_waifus_protocol::{ArrayIntInt, NormalItem, WeaponItem};

use crate::config;
use crate::logic::utils::seq_utils::{SequenceGenerator, Sequencer};
use wicked_waifus_protocol_internal::{PlayerInventoryData, PlayerInventoryWeaponData};

pub struct PlayerInventory {
    items: HashMap<i32, i32>,
    weapons_seq: SequenceGenerator<i32, AtomicI32>,
    weapons: HashMap<i32, PlayerInventoryWeaponData>,
}

pub struct ItemUsage {
    pub id: i32,
    pub quantity: i32,
}

#[derive(thiserror::Error, Debug)]
pub enum InventoryError {
    #[error("Item with id: {0} doesn't exist in inventory")]
    ItemNotFound(i32),
    #[error("There isn't enough quantity of item with id: {0}, current: {1}, requested: {2}")]
    ItemNotEnough(i32, i32, i32),
    #[error("There isn't enough quantity of some of the items required")]
    ItemsNotEnough(),
}

impl PlayerInventory {
    const UNION_EXP_ID: i32 = 1;
    const SHELL_CREDIT_ID: i32 = 2;
    const ASTRITE_ID: i32 = 3;
    const LUNITE_ID: i32 = 4;
    const WAVEPLATE_ID: i32 = 5;
    const WAVEPLATE_CRYSTAL_ID: i32 = 6;

    pub fn load_from_save(data: PlayerInventoryData) -> Self {
        Self {
            weapons_seq: SequenceGenerator::from_data(&data.weapons),
            items: data.items.clone(),
            weapons: data.weapons.clone(),
        }
    }

    pub fn build_save_data(&self) -> PlayerInventoryData {
        PlayerInventoryData {
            items: self.items.clone(),
            weapons: self.weapons.clone(),
        }
    }

    pub fn add_item(&mut self, id: i32, quantity: i32) -> i32 {
        self.add_internal(id, quantity)
    }

    pub fn add_items(&mut self, usages: &[ItemUsage]) -> HashMap<i32, i32> {
        self.add_many_internal(usages)
    }

    pub fn consume_item(&mut self, id: i32, quantity: i32) -> Result<i32, InventoryError> {
        Ok(*self
            .consume_items(&[ItemUsage {
                id,
                quantity: -quantity,
            }])?
            .get(&id)
            .unwrap())
    }

    pub fn consume_items(
        &mut self,
        usages: &[ItemUsage],
    ) -> Result<HashMap<i32, i32>, InventoryError> {
        if !self.has_enough_items(usages) {
            return Err(InventoryError::ItemsNotEnough());
        }
        Ok(self.add_many_internal(usages))
    }

    // TODO: Check if this is item or not
    #[inline(always)]
    pub fn get_union_exp(&self) -> i32 {
        self.items.get(&Self::UNION_EXP_ID).copied().unwrap_or(0)
    }

    #[inline(always)]
    pub fn add_shell_credits(&mut self, count: i32) -> i32 {
        self.add_internal(Self::SHELL_CREDIT_ID, count)
    }

    #[inline(always)]
    pub fn get_shell_credits(&self) -> i32 {
        self.items.get(&Self::SHELL_CREDIT_ID).copied().unwrap_or(0)
    }

    #[inline(always)]
    pub fn add_astrite(&mut self, count: i32) -> i32 {
        self.add_internal(Self::ASTRITE_ID, count)
    }

    #[inline(always)]
    pub fn get_astrite(&self) -> i32 {
        self.items.get(&Self::ASTRITE_ID).copied().unwrap_or(0)
    }

    #[inline(always)]
    pub fn get_lunite(&self) -> i32 {
        self.items.get(&Self::LUNITE_ID).copied().unwrap_or(0)
    }

    // TODO: Check if this is item or not
    #[inline(always)]
    pub fn get_waveplate(&self) -> i32 {
        self.items.get(&Self::WAVEPLATE_ID).copied().unwrap_or(0)
    }

    // TODO: Check if this is item or not
    #[inline(always)]
    pub fn get_waveplate_crystal(&self) -> i32 {
        self.items
            .get(&Self::WAVEPLATE_CRYSTAL_ID)
            .copied()
            .unwrap_or(0)
    }

    pub fn to_normal_item_list(&self) -> Vec<NormalItem> {
        self.items
            .iter()
            .filter(|(&id, _)| Self::WAVEPLATE_ID != id && Self::WAVEPLATE_CRYSTAL_ID != id)
            // TODO: Implement expiration
            .map(|(&id, &count)| NormalItem {
                id,
                count,
                expire_time: 0,
            })
            .collect::<Vec<_>>()
    }

    pub fn to_normal_item_list_filtered(&self, ids: &[i32]) -> Vec<NormalItem> {
        self.items
            .iter()
            .filter(|(&id, _)| ids.contains(&id))
            // TODO: Implement expiration
            .map(|(&id, &count)| NormalItem {
                id,
                count,
                expire_time: 0,
            })
            .collect::<Vec<_>>()
    }

    pub fn to_array_int_int_filtered(&self, ids: &[i32]) -> Vec<ArrayIntInt> {
        ids.iter()
            .map(|id| ArrayIntInt {
                key: *id,
                value: self.items.get(id).copied().unwrap_or(0),
            })
            .collect::<Vec<_>>()
    }

    pub fn add_weapon(
        &mut self,
        id: i32,
        func: i32,
        level: i32,
        exp: i32,
        breach: i32,
        reson: i32,
        role: Option<i32>,
    ) -> Result<i32, InventoryError> {
        let inc_id = self.weapons_seq.take_id();
        self.weapons.insert(
            inc_id,
            PlayerInventoryWeaponData {
                id,
                func_value: func,
                level,
                exp,
                breach,
                reson_level: reson,
                role_id: role.unwrap_or(0),
            },
        );
        Ok(inc_id)
    }

    pub fn remove_weapon(&mut self, id: i32) {
        self.weapons.remove(&id);
        self.weapons_seq.give_id(id);
    }

    pub fn to_weapon_item_list(&self) -> Vec<WeaponItem> {
        self.weapons
            .iter()
            .map(|(&inc_id, data)| WeaponItem {
                id: data.id,
                incr_id: inc_id,
                func_value: data.func_value,
                weapon_level: data.level,
                weapon_exp: data.exp,
                weapon_breach: data.breach,
                weapon_reson_level: data.reson_level,
                role_id: data.role_id,
            })
            .collect()
    }

    pub fn get_weapon_equip_info(&self, inc_id: i32) -> Option<(i32, i32)> {
        self.weapons
            .get(&inc_id)
            .map(|weapon_data| (weapon_data.id, weapon_data.breach))
    }

    pub fn get_weapon_equip_by_role(
        &self,
        role_id: i32,
    ) -> Option<(&i32, &PlayerInventoryWeaponData)> {
        self.weapons
            .iter()
            .find(|(_, weapon)| weapon.role_id == role_id)
    }

    pub fn swap_weapon(
        &mut self,
        incr_id: i32,
        role_id: i32,
    ) -> Result<HashMap<i32, (PlayerInventoryWeaponData, PlayerInventoryWeaponData)>, InventoryError> {
        let mut result = HashMap::new();
    
        let prev_key = self
            .weapons
            .iter()
            .find_map(|(k, w)| if w.role_id == role_id { Some(*k) } else { None })
            .ok_or(InventoryError::ItemNotFound(incr_id))?;
    
        if !self.weapons.contains_key(&incr_id) {
            return Err(InventoryError::ItemNotFound(incr_id));
        }
    
        if prev_key == incr_id {
            return Ok(HashMap::new()); 
        }
    
        let before_first = self.weapons.get(&prev_key).unwrap().clone();
        let before_second = self.weapons.get(&incr_id).unwrap().clone();
    
        let mut first = self.weapons.remove(&prev_key).expect("weapon not found");
        let mut second = self.weapons.remove(&incr_id).expect("weapon not found");
    
        // Swap role_id
        if second.role_id != 0 {
            let target_owner = second.role_id;
            second.role_id = role_id;
            first.role_id = target_owner;
        } else {
            second.role_id = role_id;
            first.role_id = 0;
        }
    
        self.weapons.insert(prev_key, first.clone());
        self.weapons.insert(incr_id, second.clone());
    
        result.insert(prev_key, (before_first, first));
        result.insert(incr_id, (before_second, second));
    
        Ok(result)
    }
    

    #[inline(always)]
    fn add_internal(&mut self, id: i32, quantity: i32) -> i32 {
        *self
            .items
            .entry(id)
            .and_modify(|count| *count += quantity)
            .or_insert(quantity)
    }

    #[inline(always)]
    fn add_many_internal(&mut self, usages: &[ItemUsage]) -> HashMap<i32, i32> {
        usages
            .iter()
            .filter(|usage| usage.quantity != 0)
            .map(|delta| (delta.id, self.add_internal(delta.id, delta.quantity)))
            .collect::<HashMap<_, _>>()
    }

    #[inline(always)]
    fn has_enough_item(&self, id: i32, quantity: i32) -> bool {
        self.items.get(&id).copied().unwrap_or(0) >= quantity
    }

    #[inline(always)]
    fn has_enough_items(&self, items_delta: &[ItemUsage]) -> bool {
        items_delta
            .iter()
            .all(|delta| self.has_enough_item(delta.id, -delta.quantity))
    }
}

impl Default for PlayerInventory {
    fn default() -> Self {
        let mut weapons_seq = SequenceGenerator::new();
        let default_unlocks = &config::get_config().default_unlocks;
        let weapons: HashMap<i32, PlayerInventoryWeaponData> =
            match default_unlocks.unlock_all_weapons {
                true => wicked_waifus_data::weapon_conf_data::iter()
                    .map(|data| {
                        let (level, breach) = if default_unlocks.unlock_all_weapons_max_level {
                            (
                                wicked_waifus_data::weapon_level_data::iter()
                                    .filter(|level_data| level_data.level_id == data.level_id)
                                    .map(|level_data| level_data.level)
                                    .max()
                                    .unwrap_or(1),
                                wicked_waifus_data::weapon_breach_data::iter()
                                    .filter(|level_data| level_data.breach_id == data.breach_id)
                                    .map(|level_data| level_data.level)
                                    .max()
                                    .unwrap_or(0),
                            )
                        } else {
                            (
                                wicked_waifus_data::weapon_level_data::iter()
                                    .filter(|level_data| level_data.level_id == data.level_id)
                                    .map(|level_data| level_data.level)
                                    .min()
                                    .unwrap_or(1),
                                wicked_waifus_data::weapon_breach_data::iter()
                                    .filter(|level_data| level_data.breach_id == data.breach_id)
                                    .map(|level_data| level_data.level)
                                    .min()
                                    .unwrap_or(0),
                            )
                        };
                        let reson_level = if default_unlocks.unlock_all_weapons_all_reson {
                            wicked_waifus_data::weapon_reson_data::iter()
                                .filter(|level_data| level_data.reson_id == data.reson_id)
                                .map(|level_data| level_data.level)
                                .max()
                                .unwrap_or(0)
                        } else {
                            wicked_waifus_data::weapon_reson_data::iter()
                                .filter(|level_data| level_data.reson_id == data.reson_id)
                                .map(|level_data| level_data.level)
                                .min()
                                .unwrap_or(0)
                        };
                        (
                            weapons_seq.take_id(),
                            PlayerInventoryWeaponData {
                                id: data.item_id,
                                func_value: 0,
                                level,
                                exp: 0,
                                breach,
                                reson_level,
                                role_id: 0,
                            },
                        )
                    })
                    .collect::<HashMap<_, _>>(),
                false => Default::default(),
            };
        Self {
            items: HashMap::new(),
            weapons_seq,
            weapons,
        }
    }
}
