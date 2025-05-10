use axum::{
    Router,
    body::Body,
    http::{Request, Response},
};

use entity::sea_orm_active_enums::UserRole;
use serde::Serialize;
use tower::ServiceExt;

use osvitarium_backend::routes::login::{Request as LoginRequest, Response as LoginResponse};

pub struct TestBuilder {
    // Routing information
    pub router: Router,

    // Request data
    pub method: String,
    pub endpoint: Option<String>,

    pub body: Option<String>,

    // Authentication data
    pub role: Option<UserRole>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl TestBuilder {
    pub fn new(router: Router) -> Self {
        TestBuilder {
            router,

            method: "GET".into(),
            endpoint: None,
            body: None,

            role: None,
            username: None,
            password: None,
        }
    }

    pub fn authenticate_as(mut self, role: UserRole) -> Self {
        self.role = Some(role);

        self
    }

    pub fn with_credentials(mut self, username: &str, password: &str) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());

        self
    }

    pub fn route(mut self, method: &str, endpoint: &str) -> Self {
        self.method = method.into();
        self.endpoint = Some(endpoint.into());

        self
    }

    pub fn with_body(mut self, body: &impl Serialize) -> Self {
        self.body = Some(serde_json::to_string(body).unwrap());

        self
    }

    pub async fn run(self) -> Response<Body> {
        let mut request_builder = Request::builder()
            .uri(
                self.endpoint
                    .expect("You MUST specify endpoint (e.g. '/login') to run test request!"),
            )
            .method(self.method.as_str())
            .header("content-type", "application/json");

        // Prepare authentication token
        if let Some(_) = self.role {
            let auth_credentials = LoginRequest {
                username: self
                    .username
                    .expect("You MUST specify 'username' if you want to authenticate"),
                password: self
                    .password
                    .expect("You MUST specify 'password' if you want to authenticate"),
            };

            let login_request = Request::builder()
                .uri("/login")
                .method("POST")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&auth_credentials).unwrap(),
                ))
                .unwrap();

            let response_body = self.router.clone().oneshot(login_request).await.unwrap();
            if response_body.status() != 200 {
                panic!("Failed to pass authorization: {}", response_body.status());
            }

            let response_body: LoginResponse = serde_json::from_slice(
                &axum::body::to_bytes(response_body.into_body(), usize::MAX)
                    .await
                    .unwrap(),
            )
            .unwrap();

            // Add authorization header
            request_builder = request_builder.header(
                "Authorization",
                format!("Bearer {}", response_body.access_token),
            );
        };

        // Build final request
        let request = if let Some(request_body) = self.body {
            request_builder.body(Body::from(request_body)).unwrap()
        } else {
            request_builder.body(Body::empty()).unwrap()
        };

        self.router.oneshot(request).await.unwrap()
    }
}
