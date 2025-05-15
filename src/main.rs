use osvitarium_backend::{app::builder::AppBuilder, config::AppConfig};

use tokio::net::TcpListener;

use clap::Parser;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = AppConfig::parse();

    let app = AppBuilder::new()
        .secret(&config.secret)
        .db(&config.db)
        .jitsi_app_id(&config.jitsi_app_id)
        .jitsi_secret(&config.jitsi_secret)
        .jitsi_kid(&config.jitsi_kid)
        .build()
        .await
        .unwrap();

    let listener = TcpListener::bind((config.host, config.port))
        .await
        .map_err(|e| {
            log::error!("Failed to bind TCP listener: {e}");

            std::process::exit(1)
        })
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
