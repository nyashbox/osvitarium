pub mod error;
pub mod state;

use tokio::net::TcpListener;

use crate::config::AppConfig;

use axum::{Router, routing::post};

pub struct App {
    listen_host: String,
    listen_port: u16,
}

impl App {
    pub async fn new(config: AppConfig) -> Self {
        App {
            listen_host: config.host,
            listen_port: config.port,
        }
    }

    async fn routes(&self) -> Router {
        Router::new()
    }

    pub async fn run(&self) {
        let listener = TcpListener::bind((self.listen_host.clone(), self.listen_port))
            .await
            .unwrap();

        let service = self.routes().await;

        axum::serve(listener, service).await.unwrap();
    }
}
