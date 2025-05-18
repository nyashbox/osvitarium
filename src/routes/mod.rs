use crate::routes::prelude::*;

pub mod courses;
pub mod login;
pub mod me;
pub mod prelude;
pub mod signup;
pub mod users;

use axum::{middleware, routing};

use crate::{middleware::auth::auth_middleware, repositories::RepositoryTrait};

use crate::routes::docs::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
            routing::get(courses::courses::get_all_courses)
                .post(courses::courses::create_new_course)
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                )),
        )
        .route(
            "/courses/{id}",
            routing::get(courses::courses::get_course_by_id)
                .delete(courses::courses::delete_course_by_id)
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                )),
        )
        .route(
            "/courses/{id}/activities",
            routing::get(courses::activity::get_course_activities)
                .post(courses::activity::create_course_activity)
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                )),
        )
        .route(
            "/courses/{id}/meeting",
            routing::get(courses::meeting::get_course_meeting_credentials)
                .post(courses::meeting::create_new_course_meeting)
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                )),
        )
        .route(
            "/me",
            routing::get(me::me::get_profile_information)
                .delete(me::me::delete_my_profile)
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                )),
        )
        .route(
            "/users/{id}",
            routing::get(users::users::get_user_description).layer(middleware::from_fn_with_state(
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

    use super::courses::activity::*;
    use super::courses::courses::*;
    use super::courses::meeting::*;
    use super::login::*;
    use super::signup::*;
    use super::users::users::*;

    use super::me::me::*;

    #[derive(OpenApi)]
    #[openapi(
        paths(
            get_all_courses,
            get_course_by_id,
            create_new_course,
            create_new_course_meeting,
            get_course_meeting_credentials,
            login_post_handler,
            signup_post_handler,
            get_profile_information,
            delete_course_by_id,
            delete_my_profile,
            get_user_description,
            get_course_activities,
            create_course_activity,
        ),
        security(
            ("jwt_token" = [])
        ),
        tags(
            (name = "Authentication", description = "Authentication endpoints"),
            (name = "Profile", description = "Profile endpoints. Requires prior authentication (see Authentication section)"),
            (name = "Courses", description = "Courses endpoints")
        ),
        modifiers(&SecurityAddon)
    )]
    pub struct ApiDoc;
}
