use std::sync::LazyLock;

use anyhow::Result;
use serde::Deserialize;

use wicked_waifus_commons::config_util::{self, TomlConfig};
use wicked_waifus_http::{
    Application,
    config::{AesSettings, NetworkSettings},
};

#[derive(Deserialize)]
pub struct ServeConfig {
    pub serve_web_path: String,
    pub serve_dir_path: String,
}

#[derive(Deserialize)]
pub struct AssetConfig {
    pub repository_url: String,
    pub reference: String,
    pub discard_local_changes: bool,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub network: NetworkSettings,
    pub encryption: AesSettings,
    pub serve: ServeConfig,
    pub asset_config: Option<AssetConfig>,
}

impl TomlConfig for ServerConfig {
    const DEFAULT_TOML: &str = include_str!("../configserver.default.toml");
}

#[tokio::main]
async fn main() -> Result<()> {
    static CONFIG: LazyLock<ServerConfig> =
        LazyLock::new(|| config_util::load_or_create("configserver.toml"));

    ::wicked_waifus_commons::splash::print_splash();
    ::wicked_waifus_commons::logging::init_axum(::tracing::Level::DEBUG);

    if let Some(asset_config) = &CONFIG.asset_config {
        wicked_waifus_asset_updater::clone_or_update_repository(
            &asset_config.repository_url,
            &CONFIG.serve.serve_dir_path,
            &asset_config.reference,
            asset_config.discard_local_changes,
        ).unwrap();
    }

    tracing::debug!(
        "Serving files from {} at {}",
        &CONFIG.serve.serve_web_path,
        &CONFIG.serve.serve_dir_path
    );

    Application::new()
        .serve_dir(&CONFIG.serve.serve_web_path, &CONFIG.serve.serve_dir_path)
        .with_encryption(&CONFIG.encryption)
        .with_logger()
        .serve(&CONFIG.network)
        .await?;

    Ok(())
}
