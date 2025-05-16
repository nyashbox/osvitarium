pub use crate::{
    app::{AppState, AppStatus as Status},
    models::*,
    repositories::course::CourseRepository,
};

pub use axum::{
    Extension, Router,
    extract::{Path, State},
    response::Json,
};

pub use entity::sea_orm_active_enums::UserRole;

pub use serde::{Deserialize, Serialize};

pub use std::sync::Arc;

pub use utoipa::ToSchema;
