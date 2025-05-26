use anyhow::Result;
use handler::{client_message_handler, game_server_connection, service_message_handler};
use session::SessionManager;
use std::sync::{Arc, LazyLock, OnceLock};
use udp_server::UdpServer;

use wicked_waifus_commons::config_util;
use config::ServerConfig;
use wicked_waifus_protokey::ServerProtoKeyHelper;

mod config;
mod handler;
mod session;
mod udp_server;

const CLIENT_PUBLIC_KEY: &str = include_str!("../security/client_public_key.pem");

#[tokio::main]
async fn main() -> Result<()> {
    static PROTOKEY_HELPER: OnceLock<ServerProtoKeyHelper> = OnceLock::new();
    static CONFIG: LazyLock<ServerConfig> =
        LazyLock::new(|| config_util::load_or_create("gateway.toml"));
    static SESSION_MGR: LazyLock<SessionManager> = LazyLock::new(SessionManager::default);

    ::wicked_waifus_commons::splash::print_splash();
    ::wicked_waifus_commons::logging::init(::tracing::Level::DEBUG);

    let protokey_helper =
        ServerProtoKeyHelper::with_public_key(&CONFIG.protokey, CLIENT_PUBLIC_KEY)?;

    let database = wicked_waifus_database::connect_to(&CONFIG.database).await?;
    wicked_waifus_database::run_migrations(&database).await?;

    game_server_connection::init(CONFIG.service_id, &CONFIG.game_server_end_point);
    client_message_handler::start_task(&SESSION_MGR);
    service_message_handler::start_task(&CONFIG.service_end_point, &SESSION_MGR).await?;

    let server = UdpServer::new(
        &CONFIG.network,
        PROTOKEY_HELPER.get_or_init(|| protokey_helper),
        CONFIG.network.kcp_crc.unwrap_or(false),
        &SESSION_MGR,
        Arc::new(database),
    )
    .await?;
    server.serve().await;

    Ok(())
}
