use osvitarium_backend::app::state::AppState;

use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait, Set};

use migration::{Migrator, MigratorTrait};

use osvitarium_backend::services::user::utils;

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

    AppState { db, secret }
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
}

/// Insert new student into the database
///
/// # Arguments
///
/// * 'username' - student username
/// * 'password' - user password
///
/// # Returns
///
/// Inserted model
pub async fn insert_student(
    db: &DatabaseConnection,
    username: &str,
    password: &str,
) -> entity::student::Model {
    let user = entity::user::ActiveModel {
        username: Set(username.to_string()),
        password: Set(utils::hash_password(password).unwrap()),
        fullname: Set("".to_string()),
        description: Set("".to_string()),
        metadata: Set("{}".into()),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap();

    entity::student::ActiveModel {
        user_id: Set(user.user_id),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap()
}
