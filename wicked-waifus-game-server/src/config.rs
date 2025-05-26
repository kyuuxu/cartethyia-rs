use std::sync::OnceLock;

use serde::Deserialize;

use wicked_waifus_commons::config_util;
use wicked_waifus_commons::config_util::TomlConfig;
use wicked_waifus_database::DatabaseSettings;
use wicked_waifus_network::config::ServiceEndPoint;

#[derive(Deserialize)]
pub struct ServiceConfig {
    pub service_id: u32,
    pub database: DatabaseSettings,
    pub service_end_point: ServiceEndPoint,
    pub gateway_end_point: ServiceEndPoint,
    pub game_server_config: GameServerConfig,
    pub asset_config: AssetConfig,
    pub default_unlocks: DefaultUnlocks,
}

#[derive(Deserialize)]
pub struct GameServerConfig {
    pub resources_path: String,
    pub load_textmaps: bool,
    pub quadrant_size: f32,
}

#[derive(Deserialize)]
pub struct AssetConfig {
    pub asset_url: String,
    pub buffer_size: usize,
}

#[derive(Deserialize)]
pub struct DefaultUnlocks {
    pub unlock_all_roles: bool,
    pub unlock_all_roles_max_level: bool,
    pub unlock_all_roles_all_sequences: bool,
    pub unlock_all_mc_elements: bool,
    pub unlock_all_weapons: bool,
    pub unlock_all_weapons_max_level: bool,
    pub unlock_all_weapons_all_reson: bool,
    pub unlock_all_adventures: bool,
    pub unlock_all_functions: bool,
    pub unlock_all_guides: bool,
    pub unlock_all_tutorials: bool,
    pub unlock_all_teleporter: bool,
    pub unlock_all_role_skins: bool,
    pub unlock_all_weapon_skins: bool,
    pub unlock_all_fly_skins: bool,
    pub unlock_all_wing_skins: bool,
}

impl TomlConfig for ServiceConfig {
    const DEFAULT_TOML: &str = include_str!("../gameserver.default.toml");
}

static CONFIG: OnceLock<ServiceConfig> = OnceLock::new();

pub fn get_config() -> &'static ServiceConfig {
    CONFIG.get_or_init(|| config_util::load_or_create("gameserver.toml"))
}