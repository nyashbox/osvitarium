use std::sync::Arc;

use axum::Router;

use sea_orm::Database;

use crate::{app::state::AppState, app::status::AppStatus as Status, routes::build_routes};

/// Represents intermediate application state that can be converted into app
/// that can be run in the future
pub struct AppBuilder {
    /// Application secret
    secret: Option<String>,
    /// Database URL
    db_url: Option<String>,
}

impl AppBuilder {
    /// Create application builder
    ///
    /// # Arguments
    ///
    /// This function takes no arguments
    ///
    /// # Returns
    ///
    /// This function returns nothing
    pub fn new() -> Self {
        AppBuilder {
            secret: None,
            db_url: None,
        }
    }

    /// Set application secret
    ///
    /// # Arguments
    ///
    /// * 'secret' - application secret
    ///
    /// # Returns
    ///
    /// This function returns nothing
    pub fn secret(mut self, secret: &str) -> Self {
        self.secret = Some(String::from(secret));

        self
    }

    /// Build database URL
    ///
    /// # Arguments
    ///
    /// * 'url' - Database URL
    ///
    /// # Returns
    ///
    /// This function returns self (ownership)
    pub fn db(mut self, url: &str) -> Self {
        self.db_url = Some(String::from(url));

        self
    }

    /// Build application
    ///
    /// # Arguments
    ///
    /// This function takes no arguments
    ///
    /// # Returns
    ///
    /// On success: Ready to run application
    /// On failure: Application status
    pub async fn build(self) -> Result<Router, Status> {
        let db_url = self.db_url.ok_or_else(|| {
            log::error!("Database URL must be specified!");

            Status::Internal(None)
        })?;

        let secret = self.secret.ok_or_else(|| {
            log::error!("Application secret MUST be specified!");

            Status::Internal(None)
        })?;

        let db = Database::connect(db_url).await.map_err(|e| {
            log::error!("Failed to create database connection: {e}");

            Status::Internal(None)
        })?;

        Ok(build_routes(Arc::new(AppState { db, secret })))
    }
}
