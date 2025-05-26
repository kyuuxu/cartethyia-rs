use wicked_waifus_commons::config_util::TomlConfig;
use serde::Deserialize;
use wicked_waifus_database::DatabaseSettings;
use wicked_waifus_http::config::NetworkSettings;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub network: NetworkSettings,
    pub database: DatabaseSettings,
    pub gateway: GatewayConfig,
}

#[derive(Deserialize)]
pub struct GatewayConfig {
    pub host: String,
    pub port: u16,
}

impl TomlConfig for ServerConfig {
    const DEFAULT_TOML: &str = include_str!("../loginserver.default.toml");
}
