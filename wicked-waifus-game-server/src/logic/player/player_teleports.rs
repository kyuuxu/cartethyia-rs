use wicked_waifus_protocol_internal::{PlayerTeleportData, PlayerTeleportsData};

use crate::config;

pub struct PlayerTeleport {
    pub id: i32,
    pub map_id: i32,
    pub entity_config_id: i64,
}

pub struct PlayerTeleports {
    pub teleports_data: Vec<PlayerTeleport>,
}

impl PlayerTeleports {
    pub fn load_from_save(data: PlayerTeleportsData) -> Self {
        Self {
            teleports_data: data
                .teleport_data
                .iter()
                .map(|teleport| PlayerTeleport {
                    id: teleport.id,
                    map_id: teleport.map_id,
                    entity_config_id: teleport.entity_config_id,
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn build_save_data(&self) -> PlayerTeleportsData {
        PlayerTeleportsData {
            teleport_data: self
                .teleports_data
                .iter()
                .map(|teleport| PlayerTeleportData {
                    id: teleport.id,
                    map_id: teleport.map_id,
                    entity_config_id: teleport.entity_config_id,
                })
                .collect::<Vec<_>>(),
        }
    }
}

impl Default for PlayerTeleports {
    fn default() -> Self {
        if config::get_config().default_unlocks.unlock_all_teleporter {
            Self {
                teleports_data: wicked_waifus_data::teleporter_data::iter()
                    .map(|teleporter| PlayerTeleport {
                        id: teleporter.id,
                        map_id: teleporter.map_id,
                        entity_config_id: teleporter.teleport_entity_config_id,
                    })
                    .collect::<Vec<_>>(),
            }
        } else {
            Self {
                teleports_data: vec![],
            }
        }
    }
}
