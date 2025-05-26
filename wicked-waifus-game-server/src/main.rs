use std::sync::{Arc, LazyLock};

use anyhow::Result;

use config::ServiceConfig;
use session::SessionManager;
use wicked_waifus_data::gacha_view_info_data;
mod config;
mod gateway_connection;
mod logic;
mod player_save_task;
mod service_message_handler;
mod session;

#[tokio::main]
async fn main() -> Result<()> {
    static SESSION_MGR: LazyLock<SessionManager> = LazyLock::new(SessionManager::default);
    let service_config: &ServiceConfig = config::get_config();

    ::wicked_waifus_commons::splash::print_splash();
    ::wicked_waifus_commons::logging::init(::tracing::Level::DEBUG);

    wicked_waifus_asset_updater::update_from_releases(
        &service_config.asset_config.asset_url,
        &service_config.game_server_config.resources_path,
        service_config.asset_config.buffer_size,
    )?;
    let bin_data_path = format!(
        "{}/BinData",
        service_config.game_server_config.resources_path
    );
    let text_map_path = format!(
        "{}/Textmap",
        service_config.game_server_config.resources_path
    );

    wicked_waifus_data::load_all_json_data(bin_data_path.as_str())?;
    if service_config.game_server_config.load_textmaps {
        for view_info in gacha_view_info_data::iter() {
            wicked_waifus_data::text_map_data::register_filter(view_info.summary_title.clone());
            wicked_waifus_data::text_map_data::register_filter(view_info.summary_describe.clone());
        }
        wicked_waifus_data::text_map_data::load_textmaps(text_map_path.as_str())?;
    }
    logic::utils::quadrant_util::initialize_quadrant_system(
        service_config.game_server_config.quadrant_size,
    );

    let database = Arc::new(wicked_waifus_database::connect_to(&service_config.database).await?);
    wicked_waifus_database::run_migrations(database.as_ref()).await?;

    logic::thread_mgr::start_logic_threads(1);

    player_save_task::start(database.clone());
    gateway_connection::init(service_config.service_id, &service_config.gateway_end_point);
    service_message_handler::run(&service_config.service_end_point, &SESSION_MGR, database).await?;

    Ok(())
}
