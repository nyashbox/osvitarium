pub mod login;
pub mod me;

use std::sync::Arc;

use axum::{Router, middleware, routing};

use crate::{
    app::state::AppState, middleware::auth::auth_middleware, repositories::user::UserRepository,
};

/// Build application router
///
/// # Arguments
///
/// * 'state' - application state
///
/// # Returns
///
/// Application router
pub fn build_routes<S>(state: Arc<AppState<S>>) -> Router
where
    S: Send + Sync + 'static + UserRepository,
{
    Router::new()
        .route("/login", routing::post(login::login_post_handler))
        .route(
            "/me",
            routing::get(me::me_get_handler).layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .with_state(state)
}
