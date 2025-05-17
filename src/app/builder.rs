use std::sync::Arc;

use axum::Router;

use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

use crate::{app::state::AppState, app::status::AppStatus as Status, routes::build_routes};

/// Represents intermediate application state that can be converted into app
/// that can be run in the future
pub struct AppBuilder {
    /// Application secret
    secret: Option<String>,
    /// Database URL
    db_url: Option<String>,

    /// Jitsi application ID
    jitsi_app_id: Option<String>,

    /// Jitsi application secret
    jitsi_secret: Option<String>,

    /// Jitsi key identifier (kid)
    jitsi_kid: Option<String>,
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
            jitsi_app_id: None,
            jitsi_secret: None,
            jitsi_kid: None,
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

    /// Set Jitsi Meet application ID
    ///
    /// # Arguments
    ///
    /// * 'app_id' - Application ID
    ///
    /// # Returns
    ///
    /// This function returns self (ownership)
    pub fn jitsi_app_id(mut self, app_id: &str) -> Self {
        self.jitsi_app_id = Some(String::from(app_id));

        self
    }

    /// Set Jitsi Meet secret
    ///
    /// # Arguments
    ///
    /// * 'secret' - Jitsi Meet secret
    ///
    /// # Returns
    ///
    /// This function returns nothing
    pub fn jitsi_secret(mut self, secret: &str) -> Self {
        self.jitsi_secret = Some(String::from(secret));

        self
    }

    /// Set Jitsi Meet key identifier (kid)
    ///
    /// # Arguments
    ///
    /// * 'kid' - Key identifier
    ///
    /// # Returns
    ///
    /// This function returns nothing
    pub fn jitsi_kid(mut self, kid: &str) -> Self {
        self.jitsi_kid = Some(String::from(kid));

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

        // Apply database migrations
        Migrator::up(&db, None).await.map_err(|e| {
            log::error!("Failed to run database migrations: {e}");

            Status::Internal(None)
        })?;

        let jitsi_app_id = self.jitsi_app_id.ok_or_else(|| {
            log::error!("Jitsi Meet application id MUST be specified!");

            Status::Internal(None)
        })?;

        let jitsi_secret = self.jitsi_secret.ok_or_else(|| {
            log::error!("Jitsi Meet secret MUST be specified!");

            Status::Internal(None)
        })?;

        let jitsi_kid = self.jitsi_kid.ok_or_else(|| {
            log::error!("Jitsi Meet key identifier MUST be specified!");

            Status::Internal(None)
        })?;

        Ok(build_routes(Arc::new(AppState {
            db,
            secret,
            jitsi_app_id,
            jitsi_secret,
            jitsi_kid,
        })))
    }
}
