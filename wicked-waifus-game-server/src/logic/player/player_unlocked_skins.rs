use crate::config;
use std::collections::HashSet;
use wicked_waifus_protocol_internal::PlayerUnlockedSkinsData;

pub struct PlayerUnlockedSkins {
    pub role_skins: HashSet<i32>,
    pub weapon_skins: HashSet<i32>,
    pub fly_skins: HashSet<i32>,
    pub wing_skins: HashSet<i32>,
}

impl PlayerUnlockedSkins {
    pub fn load_from_save(data: PlayerUnlockedSkinsData) -> Self {
        Self {
            role_skins: data.role_skins.iter().cloned().collect(),
            weapon_skins: data.weapon_skins.iter().cloned().collect(),
            fly_skins: data.fly_skins.iter().cloned().collect(),
            wing_skins: data.wing_skins.iter().cloned().collect(),
        }
    }

    pub fn build_save_data(&self) -> PlayerUnlockedSkinsData {
        PlayerUnlockedSkinsData {
            role_skins: self.role_skins.iter().cloned().collect(),
            weapon_skins: self.weapon_skins.iter().cloned().collect(),
            fly_skins: self.fly_skins.iter().cloned().collect(),
            wing_skins: self.wing_skins.iter().cloned().collect(),
        }
    }
}

impl Default for PlayerUnlockedSkins {
    fn default() -> Self {
        let unlocks = &config::get_config().default_unlocks;

        Self {
            role_skins: if unlocks.unlock_all_role_skins {
                wicked_waifus_data::role_skin_data::iter()
                    .map(|skin| skin.id)
                    .collect()
            } else {
                HashSet::new()
            },

            weapon_skins: if unlocks.unlock_all_weapon_skins {
                wicked_waifus_data::weapon_skin_data::iter()
                    .map(|skin| skin.id)
                    .collect()
            } else {
                HashSet::new()
            },

            fly_skins: if unlocks.unlock_all_fly_skins {
                wicked_waifus_data::fly_skin_config_data::iter()
                    .filter(|skin| skin.skin_type == 0)
                    .map(|skin| skin.id)
                    .collect()
            } else {
                HashSet::new()
            },

            wing_skins: if unlocks.unlock_all_wing_skins {
                wicked_waifus_data::fly_skin_config_data::iter()
                    .filter(|skin| skin.skin_type == 1)
                    .map(|skin| skin.id)
                    .collect()
            } else {
                HashSet::new()
            },
        }
    }
}
