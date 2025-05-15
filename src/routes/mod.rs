pub mod courses;
pub mod login;
pub mod me;
pub mod meeting;
pub mod signup;

use std::sync::Arc;

use axum::{Router, middleware, routing};
use docs::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    app::state::AppState, middleware::auth::auth_middleware, repositories::RepositoryTrait,
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
    S: Send + Sync + 'static + RepositoryTrait,
{
    Router::new()
        .route("/login", routing::post(login::login_post_handler))
        .route("/signup", routing::post(signup::signup_post_handler))
        .route(
            "/courses",
            routing::get(courses::get::courses_get_handler)
                .post(courses::post::courses_post_handler)
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                )),
        )
        .route(
            "/courses/{id}",
            routing::get(courses::get::courses_get_one_handler).layer(
                middleware::from_fn_with_state(state.clone(), auth_middleware),
            ),
        )
        .route(
            "/courses/{id}/meeting",
            routing::get(meeting::get::meeting_get)
                .post(meeting::post::meeting_post_create)
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                )),
        )
        .route(
            "/me",
            routing::get(me::me_get_handler).layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .with_state(state)
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
}

mod docs {
    use utoipa::{
        Modify, OpenApi,
        openapi::security::{HttpBuilder, SecurityScheme},
    };

    use super::signup::*;
    use super::me::*;
    use super::login::*;
    use super::courses::get::*;
    use super::courses::post::*;
    use super::meeting::get::*;
    use super::meeting::post::*;

    pub struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "jwt_token",
                    SecurityScheme::Http(
                        HttpBuilder::new()
                            .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                            .bearer_format("JWT")
                            .build(),
                    ),
                );
            }
        }
    }

    #[derive(OpenApi)]
    #[openapi(
        paths(
            signup_post_handler,
            me_get_handler,
            login_post_handler,
            courses_get_handler,
            courses_post_handler,
            meeting_get,
            meeting_post_create,
        ), 
        security(
            ("jwt_token" = [])
        ),
        modifiers(&SecurityAddon)
    )]
    pub struct ApiDoc;
}
