use axum::{
    Router,
    body::Body,
    http::{Request, Response},
};
use osvitarium_backend::app::state::AppState;

use sea_orm::{Database, DatabaseConnection, EntityTrait};

use migration::{Migrator, MigratorTrait};
use serde::Serialize;

use tower::ServiceExt;

/// Build application state for the integration testing
///
/// # Arguments
///
/// * 'secret' - application secret
///
/// # Returns
///
/// Application state ready for integration testing
pub async fn build_app_state(secret: &str) -> AppState<DatabaseConnection> {
    // Step 0: Get database url for testing
    let database_url = std::env::var("DATABASE_URL_TEST").expect(
        "[Integration Testing] To run integration tests you MUST specify 'DATABASE_URL_TEST'",
    );

    let secret = secret.to_string();
    // Step 1: Connect to the database
    let db = Database::connect(database_url)
        .await
        .expect("[Integration Testing] Failed to connect to the database!");

    // Step 2: Run database migrations
    Migrator::up(&db, None)
        .await
        .expect("[Integration Testing] Failed to run database migrations!");

    AppState {
        db,
        secret,
        jitsi_app_id: std::env::var("JITSI_APP_ID_TEST").unwrap(),
        jitsi_secret: std::env::var("JITSI_SECRET_TEST").unwrap(),
        jitsi_kid: std::env::var("JITSI_KID_TEST").unwrap(),
    }
}

/// Remove all data from the database (except migrations)
///
/// # Arguments
///
/// * 'db' - database connection
///
/// # Returns
///
/// This function doensn't return anything
pub async fn empty_database(db: &DatabaseConnection) {
    // Delete all entities from the database
    entity::principal::Entity::delete_many()
        .exec(db)
        .await
        .unwrap();
    entity::student::Entity::delete_many()
        .exec(db)
        .await
        .unwrap();
    entity::user::Entity::delete_many().exec(db).await.unwrap();
    entity::teacher::Entity::delete_many()
        .exec(db)
        .await
        .unwrap();

    entity::course::Entity::delete_many()
        .exec(db)
        .await
        .unwrap();

    entity::course_instructor::Entity::delete_many()
        .exec(db)
        .await
        .unwrap();

    entity::course_student::Entity::delete_many()
        .exec(db)
        .await
        .unwrap();
}

pub async fn request_post(router: Router, uri: &str, body: &impl Serialize) -> Response<Body> {
    let request = Request::builder()
        .uri(uri)
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(body).unwrap()))
        .unwrap();

    router.oneshot(request).await.unwrap()
}

pub async fn request_get(router: Router, uri: &str, body: &impl Serialize) -> Response<Body> {
    let request = Request::builder()
        .uri(uri)
        .method("GET")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(body).unwrap()))
        .unwrap();

    router.oneshot(request).await.unwrap()
}
