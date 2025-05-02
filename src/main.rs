use osvitarium_backend::config::AppConfig;

use osvitarium_backend::app::App;

use clap::Parser;

#[tokio::main]
async fn main() {
    let config = AppConfig::parse();

    let app = App::new(config).await;

    app.run().await;
}
