use std::{process, sync::LazyLock};

use anyhow::Result;

use config::{GatewayConfig, ServerConfig};
use wicked_waifus_database::PgPool;
use wicked_waifus_http::{Application, StatusCode};

mod config;
mod handler;
mod schema;

#[derive(Clone)]
pub struct ServiceState {
    pub pool: PgPool,
    pub gateway: &'static GatewayConfig,
}

#[tokio::main]
async fn main() -> Result<()> {
    static CONFIG: LazyLock<ServerConfig> =
        LazyLock::new(|| ::wicked_waifus_commons::config_util::load_or_create("loginserver.toml"));

    ::wicked_waifus_commons::splash::print_splash();
    ::wicked_waifus_commons::logging::init_axum(::tracing::Level::DEBUG);

    let Ok(pool) = wicked_waifus_database::connect_to(&CONFIG.database).await else {
        tracing::error!(
            "Failed to connect to database with connection string: {}",
            &CONFIG.database
        );
        process::exit(1);
    };

    wicked_waifus_database::run_migrations(&pool).await?;

    Application::new_with_state(ServiceState {
        pool,
        gateway: &CONFIG.gateway,
    })
        .get("/health", || async { StatusCode::OK })
        .get("/api/login", handler::handle_login_api_call)
        .with_logger()
        .serve(&CONFIG.network)
        .await?;

    Ok(())
}
